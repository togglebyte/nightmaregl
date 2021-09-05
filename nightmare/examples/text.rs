use nightmare::events::{Event, EventLoop, ButtonState, Key, LoopAction};
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
    renderer.set_shader(shader, viewport.view_projection(), &mut context);
    if let Some(colour_loc) = renderer.get_uniform("col") {
        eprintln!("setting colour");
        renderer.set_uniform(Uniform::Vec3([0.0, 0.0, 0.0]), colour_loc, &mut context);
    }

    // -----------------------------------------------------------------------------
    //     - Text -
    // -----------------------------------------------------------------------------
    let font_size = 40.0;
    let mut text = Text::from_path("examples/hack.ttf", font_size, WordWrap::NoWrap, &context)?;
    text.set_text("456Q123ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let pos = Position::new(100.0, viewport.centre().y);
    text.position(pos);

    // -----------------------------------------------------------------------------
    //     - Colours -
    // -----------------------------------------------------------------------------
    let mut colours = [0.0, 0.0, 0.0];

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
            Event::Key {
                key, 
                state: ButtonState::Pressed
            } => {
                match key {
                    Key::A => colours[0] += 0.1,
                    Key::S => colours[1] += 0.1,
                    Key::D => colours[2] += 0.1,
                    Key::Z => colours[0] -= 0.1,
                    Key::X => colours[1] -= 0.1,
                    Key::C => colours[2] -= 0.1,
                    _ => {}
                }

                for val in &mut colours {
                    if *val > 1.0 {
                        *val = 1.0;
                    }
                    if *val < 0.0 {
                        *val = 0.0;
                    }
                }

                renderer.get_uniform("col").map(|colour_loc| renderer.set_uniform(Uniform::Vec3(colours), colour_loc, &mut context));
            }
            Event::Char('q') => return LoopAction::Quit,
            _ => {}
        }

        LoopAction::Continue
    });

    Ok(())
}

