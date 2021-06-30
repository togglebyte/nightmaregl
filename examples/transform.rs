use std::time::Instant;

use nightmaregl::events::{Event, EventLoop, LoopAction};
use nightmaregl::texture::Texture;
use nightmaregl::{
    Color, Context, Position, Renderer, Result,
    Rotation, Sprite, VertexData, Viewport, Transform
};

fn main() -> Result<()> {
    // -----------------------------------------------------------------------------
    //     - Event loop -
    // -----------------------------------------------------------------------------
    let (el, mut context) = Context::builder("Best game ever!").build()?;
    let eventloop = EventLoop::<()>::new(el);

    // -----------------------------------------------------------------------------
    //     - Viewport and renderer setup -
    //     Create a viewport that covers 0,0 to window size.
    //     Set the pixel size of the renderer to 8 
    //     (meaning each pixel drawn, will be 8 times as big)
    // -----------------------------------------------------------------------------
    let window_size = context.window_size();
    let viewport = Viewport::new(Position::zero(), window_size);

    let mut renderer = Renderer::default(&mut context)?;
    renderer.pixel_size = 8;

    // -----------------------------------------------------------------------------
    //     - First sprite, texture and transform -
    //     This transform is goig to be the origin 
    //     of the next transform by translating the next transform
    //     relative to this one.
    // -----------------------------------------------------------------------------
    let buny = Texture::from_disk("examples/buny.png")?;
    let mut buny_sprite = Sprite::new(&buny);

    // Set the anchor point / pivot point to the centre of the sprite
    buny_sprite.anchor = (buny_sprite.size / 2.0).to_vector();

    let mut buny_transform = Transform::default();

    // Create a position that is the centre of the screen.
    // Because the pixel size is 8, the position has to be divided by 8.
    let buny_pos = (viewport.size().cast() / 2.0f32 / renderer.pixel_size as f32).to_vector();

    // Translate the transform to the new position.
    buny_transform.translate_mut(buny_pos);

    // -----------------------------------------------------------------------------
    //     - Second sprite, texture and transform -
    //     This is an image of an arrow facing up.
    // -----------------------------------------------------------------------------
    let arrow_texture = Texture::from_disk("examples/transform.png")?;
    let mut arrow = Sprite::new(&arrow_texture);
    arrow.anchor = (arrow.size / 2.0).to_vector();
    let mut arrow_transform = Transform::default();

    // Place this to the right of the buny sprite.
    // Since this is goign to be rendered relative to the other
    // sprite, it's not necessary to position this yet.
    arrow_transform.translate_mut(Position::new(42.0, 0.0));

    let now = Instant::now();

    eventloop.run(move |event| {
        match event {
            Event::Draw(_) => {
                context.clear(Color::grey());

                // Get the vertex data for the buny sprite
                let buny_vertex_data = VertexData::new(&buny_sprite, &buny_transform);

                // Render the buny
                let _ = renderer.render(
                    &buny,
                    &[buny_vertex_data],
                    &viewport,
                    &mut context,
                );

                // Rotate the buny sprite.
                let rot = Rotation::radians(now.elapsed().as_secs_f64().sin() as f32);
                buny_transform.rotate_mut(rot);
                    
                let mut arrow_vertex_data = VertexData::new(&arrow, &arrow_transform);
                // Make the arrow vertex data relative to the buny.
                arrow_vertex_data.make_relative(&buny_transform);

                // Render the arrow
                let _ = renderer.render(
                    &arrow_texture,
                    &[arrow_vertex_data],
                    &viewport,
                    &mut context
                );

                context.swap_buffers();
            }
            Event::Char('q') => return LoopAction::Quit,
            _ => {}
        }

        LoopAction::Continue
    });
}
