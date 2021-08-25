use nightmare::events::{Event, EventLoop, Key, LoopAction};
use nightmare::texture::Texture;
use nightmare::render2d::{Model, SimpleRenderer};
use nightmare::{
    Color, Context, Position, Result, create_model_matrix,
    Rotation, Size, Sprite, Transform, Viewport, VertexData,
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
    let mut viewport = Viewport::new(Position::zero(), window_size);
    let mut renderer = SimpleRenderer::new(&mut context, viewport.view_projection())?;

    // -----------------------------------------------------------------------------
    //     - Create a sprite -
    //     * Create a sprite from a texture.
    //     * Position the sprite in the middle of the screen.
    //     * Set the anchor in the middle of the sprite so it rotates
    //       around the middle.
    // -----------------------------------------------------------------------------
    let texture = Texture::from_disk("examples/buny.png")?;
    let mut sprite = Sprite::new(&texture);

    sprite.anchor = (sprite.size / 2.0f32).to_vector();
    sprite.size = Size::new(sprite.size.width * 4.0, sprite.size.height * 4.0);

    let mut transform = Transform::default();
    transform.translate_mut(viewport.centre().to_f32());

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

                // Move the sprite a bit...
                let mut new_pos = viewport.centre().cast::<f32>();
                new_pos += Position::new(t.sin(), t.cos()) * 200.0;
                new_pos -= Position::new(sprite.size.width, sprite.size.height);
                transform.translate_mut(new_pos);

                // ... and rotate it
                transform.rotate_mut(Rotation::radians(t / 1.0));

                let model_matrix = create_model_matrix(&sprite, &transform);

                let model = Model::new(model_matrix);
                renderer.load_data(&[model], &mut context);
                renderer.render_instanced(&mut context, 1);

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
