use nightmare::events::{Event, EventLoop, Key, LoopAction};
use nightmare::texture::Texture;
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
    let mut renderer = SimpleRenderer::<Model>::new(&mut context, viewport.view_projection())?;

    // -----------------------------------------------------------------------------
    //     - Create a sprite -
    //     * Create a sprite from a texture.
    //     * Position the sprite in the middle of the screen.
    //     * Set the anchor in the middle of the sprite so it rotates
    //       around the middle.
    // -----------------------------------------------------------------------------
    let texture = Texture::from_disk("examples/buny.png")?;
    let texture = Texture::from_disk("examples/square.png")?;
    let mut sprite = Sprite::new(&texture);
    sprite.texture_rect = Rect::new(0.0, 0.0, 0.5, 0.5);

    sprite.size = Size::new(sprite.size.x * 4.0, sprite.size.y * 4.0);
    sprite.anchor = (sprite.size / 2.0f32);

    let mut transform = Transform::from_parts(
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
                context.clear(Color::grey());

                // Sprite position
                let mut new_pos = viewport.centre();
                new_pos += Position::new(t.sin(), t.cos()) * 200.0;
                transform.isometry.translation = new_pos.into();

                // Sprite rotation
                let rot = Rotation::new(t / 1.0);
                transform.isometry.rotation = rot.into();

                // Create the model matrix
                let model_matrix = create_model_matrix(&sprite, &transform);

                // The vertex data (aka Model)
                let model = Model::new(model_matrix, sprite.texture_rect);
                renderer.load_data(&[model], &mut context);

                // Render the buny
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

    Ok(())
}
