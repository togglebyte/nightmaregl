use nightmaregl::events::{Event, EventLoop, LoopAction};
use nightmaregl::texture::Texture;
use nightmaregl::{
    Animation, Color, Context, Position, Renderer, Result ,
    VertexData, Viewport, Transform, Rotation
};

fn main() -> Result<()> {
    let (el, mut context) = Context::builder("Best game ever!").build()?;
    let eventloop = EventLoop::new(el);

    let window_size = context.window_size();
    let viewport = Viewport::new(Position::zero(), window_size);
    let mut renderer = Renderer::<VertexData>::default(&mut context)?;
    renderer.pixel_size = 8;

    let texture = Texture::<f32>::from_disk("examples/anim.png")?;
    // let mut sprite = Sprite::new(&texture);
    // sprite.size = Size::new(32, 32);
    // sprite.texture_rect = Rect::new(Point::zero(), Size::new(32, 32));
    // sprite.position = (*viewport.size() / 2).to_vector() / renderer.pixel_size;
    // sprite.anchor = (sprite.size / 2).to_vector();

    let mut animation = Animation::from_texture(&texture, 1, 3, 32, 40);
    animation.fps = 4.0;
    animation.repeat = true;

    let mut transform = Transform::default();
    let position = (*viewport.size() / 2).to_vector() / renderer.pixel_size;
    transform.translate_mut(position.to_f32());

    let now = std::time::Instant::now();
    eventloop.run(move |event| {
        match event {
            Event::Draw(dt) => {
                context.clear(Color::grey());
                let t = now.elapsed().as_secs_f64();

                transform.rotate_mut(Rotation::radians(t.sin() as f32 * 4.0 * dt));
                let vertex_data = VertexData::new(&animation.sprite, &transform);

                let _ = renderer.render(
                    &texture,
                    &[vertex_data],
                    &viewport,
                    &mut context,
                );

                animation.update(dt);
                context.swap_buffers();
            }
            Event::Char('q') => return LoopAction::Quit,
            _ => {}
        }

        LoopAction::Continue
    });
}
