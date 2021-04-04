use nightmaregl::events::{Event, Key, KeyState, LoopAction};
use nightmaregl::{Color, Context, Position, Result, Size, Animation, Sprite, Renderer, Viewport, Rotation};
use nightmaregl::texture::{Wrap, Texture};

fn main() -> Result<()> {
    // -----------------------------------------------------------------------------
    //     - Context -
    // -----------------------------------------------------------------------------
    let (eventloop, mut context) = Context::builder("Best game ever!")
        .build()?;

    // -----------------------------------------------------------------------------
    //     - Renderer and Viewport -
    // -----------------------------------------------------------------------------
    let window_size = context.window_size();
    let mut viewport = Viewport::new(Position::zero(), window_size);
    let mut renderer = Renderer::default(&mut context)?;
    renderer.pixel_size = 8;

    // -----------------------------------------------------------------------------
    //     - Create a sprite -
    //     * Create a sprite from a texture.
    //     * Position the sprite in the middle of the screen.
    //     * Set the anchor in the middle of the sprite so it rotates 
    //       around the middle.
    // -----------------------------------------------------------------------------
    let texture = Texture::from_disk("examples/buny.png")?;
    let mut sprite = Sprite::new(texture.size());
    sprite.anchor = Position::new(sprite.size.width / 2.0, sprite.size.height / 2.0);

    // -----------------------------------------------------------------------------
    //     - Event loop -
    // -----------------------------------------------------------------------------
    let now = std::time::Instant::now();
    eventloop.run(move |event| {
        match event {
            Event::Draw(dt) => {
                let t = now.elapsed().as_secs_f32();

                // Clear the screen
                context.clear(Color::grey());

                // Move the sprite a bit
                sprite.position = viewport.centre().cast::<f32>() / renderer.pixel_size as f32;
                sprite.position += Position::new(t.sin(), t.cos()) * 40.0;
                sprite.position -= Position::new(sprite.size.width / 2.0, sprite.size.height / 2.0);

                // ... and rotate it
                sprite.rotation = Rotation::radians(t / 1.0);

                // Draw the sprite
                renderer.render(
                	&texture,
                	&vec![sprite.vertex_data()],
                	&viewport,
                	&mut context
                );

                context.swap_buffers();
            }
            Event::Resize(size) => viewport.resize(size),
            Event::KeyInput { key: Key::Escape, .. } => {
                return LoopAction::Quit;
            }
            _ => {}
        }

        LoopAction::Continue
    });

    Ok(())
}
