use nightmare::events::{Event, EventLoop, LoopAction};
use nightmare::texture::Texture;
use nightmare::render2d::{Model, SimpleRenderer};
use nightmare::{
    Animation, Color, Context, Position, Result, create_model_matrix,
    Rotation, Size, Sprite, Transform, Viewport, VertexData,
};

fn main() -> Result<()> {
    let (el, mut context) = Context::builder("Best game ever!").build()?;
    let eventloop = EventLoop::<()>::new(el);

    let window_size = context.window_size();
    let viewport = Viewport::new(Position::zero(), window_size);
    let mut renderer = SimpleRenderer::new(&mut context, viewport.view_projection())?;

    let texture = Texture::<f32>::from_disk("examples/anim.png")?;

    let mut animation = Animation::from_texture(&texture, 1, 3, 32, 40);
    animation.fps = 4.0;
    animation.repeat = true;
    let mut size = animation.sprite.size;
    animation.sprite.size = Size::new(size.width * 4.0, size.height * 4.0);

    let mut transform = Transform::default();
    let position = (*viewport.size() / 2).to_vector();
    transform.translate_mut(position.to_f32());

    let now = std::time::Instant::now();
    eventloop.run(move |event| {
        match event {
            Event::Draw(dt) => {
                context.clear(Color::grey());
                let t = now.elapsed().as_secs_f32();

                transform.rotate_mut(Rotation::radians(t.sin()));
                let model_matrix = create_model_matrix(&animation.sprite, &transform);
                let model = Model::new(model_matrix);
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
