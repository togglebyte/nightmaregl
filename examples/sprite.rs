use nightmaregl::events::{Event, EventLoop, Key, LoopAction};
use nightmaregl::texture::Texture;
use nightmaregl::{
    Color, Context, Position, Renderer, Result, 
    Rotation, Size, Sprite, Transform, Viewport, VertexData
};

fn main() -> Result<()> {
    // -----------------------------------------------------------------------------
    //     - Context -
    // -----------------------------------------------------------------------------
    let (el, mut context) = Context::builder("Best game ever!")
        .resizable(false)
        .vsync(false)
        .with_size(Size::new(800, 600))
        .build()?;

    let eventloop = EventLoop::new(el);

    // -----------------------------------------------------------------------------
    //     - Renderer and Viewport -
    // -----------------------------------------------------------------------------
    let window_size = context.window_size();
    let mut viewport = Viewport::new(Position::zero(), window_size);
    let mut renderer = Renderer::default(&mut context)?;
    renderer.pixel_size = 4;

    // -----------------------------------------------------------------------------
    //     - Create a sprite -
    //     * Create a sprite from a texture.
    //     * Position the sprite in the middle of the screen.
    //     * Set the anchor in the middle of the sprite so it rotates
    //       around the middle.
    // -----------------------------------------------------------------------------
    let texture = Texture::from_disk("examples/buny.png")?;
    let mut sprite = Sprite::new(&texture);

    sprite.anchor = (sprite.size / 2.0).to_vector();

    let mut transform = Transform::new();
    // transform.translate(viewport.centre().to_f32() / renderer.pixel_size as f32);

    // -----------------------------------------------------------------------------
    //     - Event loop -
    // -----------------------------------------------------------------------------
    let now = std::time::Instant::now();
    eventloop.run(move |event| {
        match event {
            Event::Draw(_dt) => {
                let t = now.elapsed().as_secs_f32();

                // Clear the screen
                context.clear(Color::grey());

                // Move the sprite a bit
                let mut new_pos = viewport.centre().cast::<f32>() / renderer.pixel_size as f32;
                new_pos += Position::new(t.sin(), t.cos()) * 20.0;
                new_pos -= Position::new(sprite.size.width, sprite.size.height);
                let mut transform = transform.translate(Position::new(50.0, 50.0));

                // ... and rotate it
                let transform = transform.rotate(Rotation::radians(t / 1.0));

                let vertex_data = VertexData::new(&sprite, &transform);

                // Draw the sprite
                let res = renderer.render(
                    &texture,
                    &[vertex_data],
                    &viewport,
                    &mut context,
                );

                if let Err(e) = res {
                    eprintln!("error rendering: {:?}", e);
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
}
