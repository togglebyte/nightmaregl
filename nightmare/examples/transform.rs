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
    let viewport = Viewport::new(Position::zeros(), window_size);

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
    buny_sprite.anchor = (buny_sprite.size / 2.0);

    // Create the transformation that will:
    // * Move
    // * Rotate
    // * Scale
    // The buny
    let mut buny_transform = Transform::identity();

    // Get the centre of the screen
    let buny_pos = (viewport.size() / 2.0);

    // Scale the buny four times, and translate the 
    // transform to the new position.
    // It is important that the scale / translation happens in this order.
    buny_transform.append_scaling_mut(4.0); 
    buny_transform.append_translation_mut(&buny_pos.into());

    // -----------------------------------------------------------------------------
    //     - Second sprite, texture and transform -
    //     This is an image of an arrow facing up.
    // -----------------------------------------------------------------------------
    let arrow_texture = Texture::from_disk("examples/transform.png")?;
    let mut arrow = Sprite::new(&arrow_texture);
    arrow.z_index = 1;
    arrow.anchor = (arrow.size / 2.0);
    let mut arrow_transform = Transform::identity();

    // Place this to above the buny sprite.
    // Note that the values for the position is relative to the buny sprite
    // as the final transform for the arrow is:
    // buny_transform * arrow transform

    let now = Instant::now();

    eventloop.run(move |event| {
        match event {
            Event::Draw(_) => {
                context.clear(Color::grey());

                // Get the vertex data for the buny sprite
                let bunny_model = create_model_matrix(&buny_sprite, &buny_transform);
                let model = Model::new(bunny_model, buny_sprite.texture_rect);

                // Render the buny
                buny.bind();
                renderer.load_data(&[model], &mut context);
                renderer.render_instanced(&mut context, 1);

                // Rotate the buny sprite.
                let rot = Rotation::new(now.elapsed().as_secs_f64().sin() as f32);
                // buny_transform.append_rotation_mut(&rot.into());
                buny_transform.isometry.rotation = rot.into();

                // Translate the arrow transform before multiplying it with the 
                // buny transform, so it rotates around the bunys axis
                arrow_transform.isometry.translation = Position::new(0.0, 32.0 + 16.0).into();
                
                // Make the arrow vertex data relative to the buny.
                let mut at = buny_transform * arrow_transform;
                let mut arrow_model = create_model_matrix(&arrow, &at);

                let model = Model::new(arrow_model, arrow.texture_rect);

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
