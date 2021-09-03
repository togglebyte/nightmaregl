use std::time::Instant;

use nightmare::events::{Event, EventLoop, LoopAction};
use nightmare::render2d::{Model, SimpleRenderer};
use nightmare::texture::Texture;
use nightmare::{
    Color, Context, Position, Result, Vector, create_model_matrix,
    Rotation, Sprite, VertexData, Viewport, Transform
};

fn main() -> Result<()> {
    // -----------------------------------------------------------------------------
    //     - Event loop -
    // -----------------------------------------------------------------------------
    let (el, mut context) = Context::builder("Best game ever!").build()?;
    let eventloop = EventLoop::<()>::new(el);

    // -----------------------------------------------------------------------------
    //     - Viewport and renderer setup -
    //     Create a viewport that covers 0,0 to window size.
    //     Set the pixel size of the renderer to 8 
    //     (meaning each pixel drawn, will be 8 times as big)
    // -----------------------------------------------------------------------------
    let window_size = context.window_size();
    let viewport = Viewport::new(Position::zero(), window_size);

    let mut renderer = SimpleRenderer::new(&mut context, viewport.view_projection())?;

    // -----------------------------------------------------------------------------
    //     - First sprite, texture and transform -
    //     This transform is goig to be the origin 
    //     of the next transform by translating the next transform
    //     relative to this one.
    // -----------------------------------------------------------------------------
    let buny = Texture::from_disk("examples/buny.png")?;
    let mut buny_sprite = Sprite::new(&buny);

    // Set the anchor point / pivot point to the centre of the sprite
    buny_sprite.anchor = (buny_sprite.size / 2.0).to_vector();

    let mut buny_transform = Transform::default();
    // buny_transform.scale_mut(Vector::new(8.0, 8.0));

    // Create a position that is the centre of the screen.
    // Because the pixel size is 8, the position has to be divided by 8.
    let buny_pos = (viewport.size().cast() / 2.0f32).to_vector();

    // Translate the transform to the new position.
    buny_transform.translate_mut(buny_pos);

    // -----------------------------------------------------------------------------
    //     - Second sprite, texture and transform -
    //     This is an image of an arrow facing up.
    // -----------------------------------------------------------------------------
    let arrow_texture = Texture::from_disk("examples/transform.png")?;
    let mut arrow = Sprite::new(&arrow_texture);
    arrow.z_index = 1;
    // arrow.anchor = (arrow.size / 2.0).to_vector();
    let mut arrow_transform = Transform::default();

    // Place this to the right of the buny sprite.
    // Note that the values for the position reflects the original
    // size and anchor point of the buny sprite.
    arrow_transform.translate_mut(Position::new(1.0, 0.0));

    let now = Instant::now();

    eventloop.run(move |event| {
        match event {
            Event::Draw(_) => {
                context.clear(Color::grey());

                // Get the vertex data for the buny sprite
                let bunny_model = create_model_matrix(&buny_sprite, &buny_transform);
                let model = Model::new(bunny_model);

                // Render the buny
                buny.bind();
                renderer.load_data(&[model], &mut context);
                renderer.render_instanced(&mut context, 1);

                // Rotate the buny sprite.
                // let rot = Rotation::radians(now.elapsed().as_secs_f64().sin() as f32);
                // buny_transform.rotate_mut(rot);

                let mut arrow_model = create_model_matrix(&arrow, &arrow_transform);
                // Make the arrow vertex data relative to the buny.
                arrow_model = bunny_model * arrow_model;
                let model = Model::new(arrow_model);
                // arrow_vertex_data.make_relative(&buny_transform);

                // Render the arrow
                arrow_texture.bind();
                renderer.load_data(&[model], &mut context);
                renderer.render_instanced(&mut context, 1);

                context.swap_buffers();
            }
            Event::Char('q') => return LoopAction::Quit,
            _ => {}
        }

        LoopAction::Continue
    });
}
