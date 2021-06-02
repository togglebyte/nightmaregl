#![deny(missing_docs)]
//! # Text rendering
//! This is a hot mess
use std::path::Path;
use std::sync::Arc;
use std::fs::read as read_file;

use rusttype::gpu_cache::Cache;
use rusttype::{Font as RustTypeFont, Point, PositionedGlyph, Scale, GlyphId};
use unicode_segmentation::UnicodeSegmentation;

use crate::errors::{NightmareError, Result};
use crate::renderer::default::VertexData;
use crate::texture::Texture;
use crate::{Context, Position, Size, Vector, Sprite, Transform};

// -----------------------------------------------------------------------------
//     - Word wrapping -
// -----------------------------------------------------------------------------
/// Word wrapping
#[derive(Debug, Copy, Clone)]
pub enum WordWrap {
    /// Break a line on a word boundary
    Normal(u32),

    /// Don't do any word wrapping
    NoWrap,
}

// -----------------------------------------------------------------------------
//     - Text -
// -----------------------------------------------------------------------------
/// Used to draw text.
/// Generates text sprites and texture.
///
/// ```
/// # use nightmaregl::Context;
/// # fn run(context: Context) {
/// use nightmaregl::text::{Text, WordWrap};
///
/// let max_width = 1024;
/// let font_size = 18.0;
/// let text = Text::from_path(
///     "/usr/share/fonts/TTF/Hack-Regular.ttf",
///     font_size,
///     WordWrap::Normal(max_width),
///     &context
/// ).unwrap();
///
/// let texture = text.texture();
/// let vertex_data = text.vertex_data();
///
/// # }
/// ```
pub struct Text {
    font: Arc<Font>,
    wrap: WordWrap,
    cache: FontCache,
    sprites: Vec<(Sprite<f32>, Transform<f32>)>,
    position: Position<f32>,
    caret: Point<f32>,
    previous_glyph_id: Option<GlyphId>
}

impl Text {
    /// Create a text object from a path
    pub fn from_path(path: impl AsRef<Path>, font_size: f32, wrap: WordWrap, context: &Context) -> Result<Self> {
        let scale_factor = context.window().scale_factor() as f32;
        let font = Font::from_path(path, scale_factor * font_size)?;
        let inst = Self::from_font(Arc::new(font), wrap);
        Ok(inst)
    }

    /// Create a `Text` from an existing [struct.Font](Font) instance.
    /// Use this to avoid loading the same font and size multiple times.
    pub fn from_font(font: Arc<Font>, wrap: WordWrap) -> Self {
        Self {
            font,
            wrap,
            cache: FontCache::new(Size::new(512.0, 512.0)),
            sprites: Vec::new(),
            position: Position::zero(),
            caret: Point {x: 0.0, y: 0.0, },
            previous_glyph_id: None,
        }
    }

    /// Get a copy of the font
    pub fn font(&self) -> Arc<Font> {
        Arc::clone(&self.font)
    }

    /// Set the text and generate the sprites and correct texture.
    pub fn set_text(&mut self, text: impl AsRef<str>) -> Result<()> {
        self.caret = Point { x: 0.0, y: 0.0 };
        self.previous_glyph_id = None;
        self.layout(text.as_ref())?;
        Ok(())
    }

    /// Set the position of the font.
    pub fn position(&mut self, position: Position<f32>) {
        self.position = position;
        self.sprites.iter_mut().for_each(|(_, transform)| {
            transform.translate_mut(position);
        });
    }

    /// The texture for the font
    pub fn texture(&self) -> &Texture<f32> { 
        &self.cache.texture
    }

    /// Vertex data used to position the font
    pub fn vertex_data(&self) -> Vec<VertexData> {
        self.sprites.iter().map(|(s, t)| VertexData::new(s, t)).collect()
    }

    /// Current caret
    pub fn caret(&self) -> Position<f32> {
        Position::new(self.caret.x, self.caret.y)
    }

}

// -----------------------------------------------------------------------------
//     - Text layout -
//     This handles all the layouts of text
// -----------------------------------------------------------------------------
impl Text {
    fn layout(&mut self, text: &str) -> Result<()> {
        let glyphs = match self.wrap {
            WordWrap::Normal(width) => self.layout_normal_wrap(width, text),
            WordWrap::NoWrap => self.no_wrap(text),
        };

        // Cache the glyphs
        for glyph in &glyphs {
            self.cache.inner.queue_glyph(0, glyph.clone());
        }

        // Run this for every cached glyph.
        // Write all the cached glyphs to a texture
        let texture = &mut self.cache.texture;
        self.cache.inner.cache_queued(|rect, data| {
            texture.write_region(
                Position::new(rect.min.x, rect.min.y).cast(),
                Size::new(rect.width(), rect.height()).cast(),
                data,
            );
        })?;

        self.sprites = glyphs
            .iter()
            .filter_map(|c| self.cache.inner.rect_for(0, c).ok())
            .flatten()
            .map(|(uv, vert)| {
                let mut sprite = Sprite::new(&self.cache.texture);
                let mut transform = Transform::default();
                let scale = self.cache.size.width;
                let tex_offset = crate::Point::new(uv.min.x as f32, uv.min.y as f32).cast() * scale;
                let size = Size::new(uv.width(), uv.height());
                let pos = Position::new(vert.min.x, -vert.max.y) + self.position.cast();

                sprite.texture_rect.origin = tex_offset;
                sprite.texture_rect.size = size.cast() * scale;
                sprite.size = size;

                transform.translate_mut(pos.cast());
                transform.scale = Vector::new(scale, scale);

                (sprite, transform)
            })
            .collect::<Vec<_>>();

        Ok(())
    }

    fn position_text<'a>(
        &mut self,
        text: &str,
        wrap: WordWrap,
    ) -> Option<Vec<PositionedGlyph<'a>>> {
        let mut glyphs = Vec::with_capacity(text.chars().count());

        for c in text.chars() {
            if c.is_control() {
                if c == '\r' {
                    continue;
                }

                if c == '\n' {
                    self.caret = Point {
                        x: 0.0,
                        y: self.caret.y + self.font.advance_height,
                    };
                    continue;
                }
            }

            let base_glyph = self.font.inner.glyph(c);

            // If this is not the first character, advance the caret,
            // taking kerning into consideration.
            if let Some(prev_id) = self.previous_glyph_id {
                self.caret.x += self
                    .font
                    .inner
                    .pair_kerning(self.font.scale, prev_id, base_glyph.id());
            }

            self.previous_glyph_id = Some(base_glyph.id());

            let glyph = base_glyph.scaled(self.font.scale).positioned(self.caret);

            // Make sure that the glyph fits if normal word wrapping is done.
            // Update the caret and return None.
            if let Some(bb) = glyph.pixel_bounding_box() {
                match wrap {
                    WordWrap::Normal(max_width) if bb.max.x > max_width as i32 => {
                        self.caret = Point {
                            x: 0.0,
                            y: self.caret.y + self.font.advance_height,
                        };
                        return None
                    }
                    WordWrap::NoWrap | WordWrap::Normal(_) => {}
                }
            }

            // Advance the caret for the next character
            self.caret.x += glyph.unpositioned().h_metrics().advance_width;

            // Done
            glyphs.push(glyph);
        }

        Some(glyphs)
    }

    fn layout_normal_wrap<'a>(
        &mut self,
        width: u32,
        text: &str,
    ) -> Vec<PositionedGlyph<'a>> {
        let mut glyphs = Vec::with_capacity(text.chars().count());
        let words = text.split_word_bounds().collect::<Vec<_>>();

        let wrap = WordWrap::Normal(width);
        for word in words {
            let new_glyphs = match self.position_text(word, wrap) {
                Some(g) => Some(g),
                None => self.position_text(word, wrap),
            };

            if let Some(mut g) = new_glyphs {
                glyphs.append(&mut g);
            }
        }

        glyphs
    }

    fn no_wrap<'a>(&mut self, text: &str) -> Vec<PositionedGlyph<'a>> {
        let mut glyphs = Vec::with_capacity(text.chars().count());
        let words = text.split_word_bounds().collect::<Vec<_>>();

        let wrap = WordWrap::NoWrap;
        for word in words {
            let new_glyphs = match self.position_text(word, wrap) {
                Some(g) => Some(g),
                None => self.position_text(word, wrap),
            };

            if let Some(mut g) = new_glyphs {
                glyphs.append(&mut g);
            }
        }

        glyphs
    }
}

// -----------------------------------------------------------------------------
//     - Font -
// -----------------------------------------------------------------------------
/// A font
pub struct Font {
    scale: Scale,
    inner: RustTypeFont<'static>,
    advance_height: f32,
}

impl Font {
    /// Create a font from a font path.
    /// This will perform disk i/o and is not recommended to run in the middle
    /// of something critical.
    pub fn from_path(path: impl AsRef<Path>, font_size: f32) -> Result<Self> {
        let font_data = read_file(path)?;

        let font = match RustTypeFont::try_from_vec(font_data) {
            Some(f) => f,
            None => return Err(NightmareError::FailedToLoadFont),
        };

        let scale = Scale::uniform(font_size);
        let v_metrics = font.v_metrics(scale);
        let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

        let inst = Self {
            inner: font,
            scale,
            advance_height,
        };

        Ok(inst)
    }
}

// -----------------------------------------------------------------------------
//     - Font cache -
// -----------------------------------------------------------------------------
struct FontCache {
    inner: Cache<'static>,
    size: Size<f32>,
    texture: Texture<f32>,
}

impl FontCache {
    fn new(size: Size<f32>) -> Self {
        let cache = {
            let size = size.cast();
            Cache::builder().dimensions(size.width, size.height).build()
        };

        Self {
            inner: cache,
            size,
            texture: Texture::<f32>::new().empty_text(size),
        }
    }
}
