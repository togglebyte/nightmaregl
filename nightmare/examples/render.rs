use nightmaregl::events::{Event, EventLoop, LoopAction};
use nightmaregl::text::{Text, WordWrap};
use nightmaregl::{
    Color, Context, Position, Renderer, Result, Sprite, 
    Texture, Transform, VertexData, Viewport, Vector
};

fn main() -> Result<()> {
    let (el, mut context) = Context::builder("Best game ever!").build()?;
    let eventloop = EventLoop::<()>::new(el);

    let window_size = context.window_size();
    let viewport = Viewport::new(Position::zero(), window_size);
    let renderer = Renderer::default_font(&mut context)?;

    // -----------------------------------------------------------------------------
    //     - Text -
    // -----------------------------------------------------------------------------
    let font_size = 72.0;
    let mut text = Text::from_path(
        "/usr/share/fonts/TTF/Hack-Regular.ttf",
        font_size,
        WordWrap::NoWrap,
        &context,
    )?;
    text.position(viewport.centre().cast());
    text.set_text("Hello")?;

    // -----------------------------------------------------------------------------
    //     - Sprite / texture  -
    // -----------------------------------------------------------------------------
    let texture = Texture::from_disk("examples/buny.png")?;
    let mut sprite = Sprite::<i32>::new(&texture);
    sprite.anchor = (sprite.size / 2).to_vector();
    let mut transform = Transform::default();
    transform.scale = Vector::new(8, 8);
    transform.translate_mut(Position::new(100, 100));

    // -----------------------------------------------------------------------------
    //     - Event loop -
    // -----------------------------------------------------------------------------
    eventloop.run(move |event| {
        match event {
            Event::Draw(_dt) => {
                context.clear(Color::black());

                renderer
                    .render(
                        &text.texture(),
                        &text.vertex_data(),
                        &viewport,
                        &mut context,
                    )
                    .unwrap();

                renderer
                    .render(
                        &texture,
                        &[VertexData::new(&sprite, &transform)],
                        &viewport,
                        &mut context,
                    )
                    .unwrap();

                context.swap_buffers();
            }
            Event::Char('q') => return LoopAction::Quit,
            _ => {}
        }

        LoopAction::Continue
    });
}
