use nightmare::events::{Event, EventLoop, Key, LoopAction};
use nightmare::texture::Texture;
use nightmare::text::{WordWrap, Text, default_font_shader};
use nightmare::render2d::{Model, SimpleRenderer};
use nightmare::{
    Color, Context, Position, Result, create_model_matrix,
    Rotation, Size, Sprite, Transform, Viewport, VertexData,
    Scale, Vector, Rect
};

fn main() -> Result<()> {
    // -----------------------------------------------------------------------------
    //     - Context -
    // -----------------------------------------------------------------------------
    let (el, mut context) = Context::builder("Best game ever!")
        // .resizable(false)
        // .vsync(false)
        // .with_size(Size::new(800, 600))
        .build()?;

    let eventloop = EventLoop::<()>::new(el);

    // -----------------------------------------------------------------------------
    //     - Renderer and Viewport -
    // -----------------------------------------------------------------------------
    let window_size = context.window_size();
    let mut viewport = Viewport::new(Position::zeros(), window_size);
    let mut renderer_text = SimpleRenderer::<Model>::new(&mut context, viewport.view_projection())?;
    let mut renderer = SimpleRenderer::<Model>::new(&mut context, viewport.view_projection())?;

    // -----------------------------------------------------------------------------
    //     - Text shader -
    // -----------------------------------------------------------------------------
    let shader = default_font_shader()?;
    renderer_text.set_shader(shader, viewport.view_projection());

    // -----------------------------------------------------------------------------
    //     - Text -
    // -----------------------------------------------------------------------------
    let font_size = 40.0;
    let mut text = Text::from_path("examples/hack.ttf", font_size, WordWrap::NoWrap, &context)?;
    text.set_text("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    text.position(viewport.centre());

    // -----------------------------------------------------------------------------
    //     - Throw away buny -
    // -----------------------------------------------------------------------------
    let texture = Texture::from_disk("examples/buny.png")?;
    let mut sprite = Sprite::new(&texture);

    sprite.size = Size::new(sprite.size.x * 4.0, sprite.size.y * 4.0);
    sprite.anchor = (sprite.size / 2.0f32);

    let mut buny_transform = Transform::from_parts(
        viewport.centre().into(),
        Rotation::new(0.0).into(),
        1.0,
    );

    // -----------------------------------------------------------------------------
    //     - Event loop -
    // -----------------------------------------------------------------------------
    let now = std::time::Instant::now();
    eventloop.run(move |event| {
        match event {
            Event::Draw(_dt) => {
                let t = now.elapsed().as_secs_f32();

                // Clear the screen
                context.clear(Color { r: 0.1, g: 0.1, b: 0.2, a: 1.0 });

                // Text
                {
                    // The vertex data (aka Model)
                    let models = text.models();
                    renderer_text.load_data(&models, &mut context);

                    text.texture().bind();

                    // Render text
                    renderer_text.render_instanced(&mut context, models.len());
                }


                // Buny rendering
                {
                    // Create the model matrix
                    let bun_matrix = create_model_matrix(&sprite, &buny_transform);

                    // The vertex data (aka Model)
                    let model = Model::new(bun_matrix, sprite.texture_rect);
                    eprintln!("bun: {}", model.mat);
                    renderer.load_data(&[model], &mut context);
                    texture.bind();
                    // renderer.render_instanced(&mut context, 1);
                }

                context.swap_buffers();
            }
            Event::Resize(size) => viewport.resize(size),
            Event::Key {
                key: Key::Escape, ..
            } => return LoopAction::Quit,
            Event::Char('q') => return LoopAction::Quit,
            _ => {}
        }

        LoopAction::Continue
    });

    Ok(())
}

