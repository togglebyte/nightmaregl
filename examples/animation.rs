use nightmaregl::events::{Event, Key, KeyState, LoopAction};
use nightmaregl::texture::{Texture, Wrap};
use nightmaregl::{
    Animation, Color, Context, Position, Rect, Renderer, Result, Rotation, Size, Sprite,
    VertexData, Viewport, Point,
};

fn main() -> Result<()> {
    let (eventloop, mut context) = Context::builder("Best game ever!").build()?;

    let window_size = context.window_size();
    let mut viewport = Viewport::new(Position::zero(), window_size);
    let mut renderer = Renderer::<VertexData>::default(&mut context)?;
    renderer.pixel_size = 8;

    let texture = Texture::from_disk("examples/anim.png")?;
    let mut sprite = Sprite::new(&texture);
    sprite.size = Size::new(32, 32);
    sprite.texture_rect = Rect::new(Point::zero(), Size::new(32, 32));
    sprite.position = (*viewport.size() / 2).to_vector() / renderer.pixel_size;
    sprite.anchor = (sprite.size / 2).to_vector();

    let mut animation = Animation::new(sprite, 1, 3, 32);
    animation.fps = 4.0;
    animation.should_loop = true;

    eventloop.run(move |event| {
        match event {
            Event::Draw(dt) => {
                context.clear(Color::grey());

                renderer.render(
                    &texture,
                    &[animation.vertex_data()],
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

    Ok(())
}
