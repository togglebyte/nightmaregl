use nightmaregl::events::{Event, LoopAction};
use nightmaregl::text::{Text, WordWrap};
use nightmaregl::{Color, Context, Position, Renderer, Result, Viewport};

fn main() -> Result<()> {
    let (eventloop, mut context) = Context::builder("Best game ever!").build()?;

    let window_size = context.window_size();
    let viewport = Viewport::new(Position::zero(), window_size);
    let renderer = Renderer::default_font(&mut context)?;

    let font_size = 72.0;
    let mut text = Text::from_path(
        "/usr/share/fonts/TTF/Hack-Regular.ttf",
        font_size,
        WordWrap::NoWrap,
        &context,
    )?;
    text.position(viewport.centre().cast());
    text.set_text("Hello")?;

    eventloop.run(move |event| {
        match event {
            Event::Draw(_dt) => {
                context.clear(Color::black());

                let _ = renderer.render(
                    &text.texture(),
                    &text.vertex_data(),
                    &viewport,
                    &mut context,
                );

                context.swap_buffers();
            }
            Event::Char('q') => return LoopAction::Quit,
            _ => {}
        }

        LoopAction::Continue
    });
}
