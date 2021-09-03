use nightmare::events::{Event, EventLoop, LoopAction};
use nightmare::texture::Texture;
use nightmare::render2d::{Model, SimpleRenderer};
use nightmare::{
    Animation, Color, Context, Position, Result, create_model_matrix,
    Rotation, Size, Sprite, Transform, Viewport, VertexData, Rect
};

fn main() -> Result<()> {
    let (el, mut context) = Context::builder("Best game ever!").build()?;
    let eventloop = EventLoop::<()>::new(el);

    let window_size = context.window_size();
    let viewport = Viewport::new(Position::zeros(), window_size);
    let mut renderer = SimpleRenderer::new(&mut context, viewport.view_projection())?;

    let texture = Texture::from_disk("examples/anim.png")?;
    let mut sprite = Sprite::new(&texture);
    sprite.size = Size::new(sprite.size.x / 3.0 * 4.0, sprite.size.y * 4.0);
    // sprite.texture_rect = Rect::new(0.0, 0.0, 1.0 / 3.0, 1.0);

    let mut animation = Animation::from_sprite(sprite, 1, 3, 32, 40);
    animation.fps = 4.0;
    animation.repeat = true;
    let mut size = animation.sprite.size;

    let mut transform = Transform::from_parts(
        viewport.centre().into(),
        Rotation::new(0.0).into(),
        1.0,
    );

    let now = std::time::Instant::now();

    eventloop.run(move |event| {
        match event {
            Event::Draw(dt) => {
                context.clear(Color::grey());
                let t = now.elapsed().as_secs_f32();

                // transform.rotate_mut(Rotation::radians(t.sin()));
                let model_matrix = create_model_matrix(&sprite, &transform);
                let model = Model::new(model_matrix, animation.sprite.texture_rect);
                renderer.load_data(&[model], &mut context);
                renderer.render_instanced(&mut context, 1);

                animation.update(dt);
                context.swap_buffers();
            }
            Event::Char('q') => return LoopAction::Quit,
            _ => {}
        }

        LoopAction::Continue
    });
}
