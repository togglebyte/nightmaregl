use nightmare::events::{Event, EventLoop, Key, LoopAction};
use nightmare::texture::Texture;
use nightmare::text::{WordWrap, Text, default_font_shader};
use nightmare::render2d::{Model, SimpleRenderer, Uniform};
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
    let mut renderer = SimpleRenderer::<Model>::new(&mut context, viewport.view_projection())?;

    // -----------------------------------------------------------------------------
    //     - Text shader -
    // -----------------------------------------------------------------------------
    let shader = default_font_shader()?;
    renderer.set_shader(shader, viewport.view_projection());
    let colour_loc = renderer.get_uniform("red").unwrap();
    renderer.set_uniform(Uniform::Float(10.5), colour_loc);

    // -----------------------------------------------------------------------------
    //     - Text -
    // -----------------------------------------------------------------------------
    let font_size = 40.0;
    let mut text = Text::from_path("examples/hack.ttf", font_size, WordWrap::NoWrap, &context)?;
    text.set_text("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    text.position(viewport.centre());

    // -----------------------------------------------------------------------------
    //     - Event loop -
    // -----------------------------------------------------------------------------
    let now = std::time::Instant::now();
    eventloop.run(move |event| {
        match event {
            Event::Draw(_dt) => {
                let _t = now.elapsed().as_secs_f32();

                // Clear the screen
                context.clear(Color { r: 0.1, g: 0.1, b: 0.2, a: 1.0 });

                // Text
                {
                    // The vertex data (aka Model)
                    let models = text.models();
                    renderer.load_data(&models, &mut context);

                    text.texture().bind();

                    // Render text
                    renderer.render_instanced(&mut context, models.len());
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

