use nightmaregl::events::{Event, EventLoop, Key, KeyState, LoopAction};
use nightmaregl::pixels::{Pixel, Pixels};
use nightmaregl::texture::{Texture, Wrap};
use nightmaregl::{
    Animation, Color, Context, Position, Renderer, Result,
    Rotation, Size, Sprite, VertexData, Viewport, Transform
};

fn main() -> Result<()> {
    let (el, mut context) = Context::builder("Best game ever!")
        .resizable(false)
        .with_size(Size::new(150, 150))
        .build()?;
    let eventloop = EventLoop::new(el);

    let window_size = context.window_size();
    let mut viewport = Viewport::new(Position::zero(), window_size);
    let mut renderer = Renderer::<VertexData>::default(&mut context)?;
    // renderer.pixel_size = 8 * 4;

    // Black box
    let trans_texture = Texture::from_disk("examples/transform.png")?;
    let mut black_box = Sprite::<f32>::new(&trans_texture);
    black_box.anchor = (black_box.size / 2.0).to_vector();
    let mut bb_t = Transform::new();
    bb_t.rotate_mut(Rotation::radians(std::f32::consts::PI));
    // bb_transform.translate_mut((*viewport.size() / 2).to_vector());

    let buny = Texture::from_disk("examples/buny.png")?;
    let mut buny_sprite = Sprite::new(&buny);
    buny_sprite.anchor = (buny_sprite.size / 2.0).to_vector();
    let mut buny_t = Transform::new();
    buny_t.translate_mut((viewport.size().cast() / 2.0f32).to_vector());
    // buny_t.rotate_mut(Rotation::radians(std::f32::consts::PI / 2.0));

    eventloop.run(move |event| {
        match event {
            Event::Draw(dt) => {
                context.clear(Color::grey());
                // let buny_vertex_data = buny_sprite.vertex_data_scaled(renderer.pixel_size as f32);
                let buny_vertex_data = VertexData::new(&buny_sprite, &buny_t);

                renderer.render(
                    &buny,
                    &[buny_vertex_data],
                    &viewport,
                    &mut context,
                );

                // let vertex_data = buny_sprite.transform(&black_box);
                // let vertex_data = black_box.vertex_data();

                // eprintln!("box : {}", vertex_data.model);

                // return LoopAction::Quit;

                let bb_t = buny_t.rotate(bb_t);
                let bb_vertex_data = VertexData::new(&black_box, &bb_t);
                renderer.render(
                    &trans_texture,
                    &[bb_vertex_data],
                    &viewport,
                    &mut context
                );

                context.swap_buffers();
            }
            Event::Char('q') => return LoopAction::Quit,
            _ => {}
        }

        LoopAction::Continue
    });

    Ok(())
}
