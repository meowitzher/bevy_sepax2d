use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_prototype_lyon::prelude::*;

use sepax2d::prelude::*;

use crate::components::{Movable, Sepax};

pub struct SepaxPlugin;

impl Plugin for SepaxPlugin
{

    fn build(&self, app: &mut App)
    {

        app
        .add_system_to_stage
        (

            CoreStage::PostUpdate,
            clear_correction_system

        )
        .add_system_to_stage
        (

            CoreStage::PostUpdate,
            update_movable_system.after(clear_correction_system)

        )
        .add_system_to_stage
        (
            
            CoreStage::PostUpdate, 
            collision_system.after(update_movable_system)

        );

        #[cfg(feature = "debug")]
        app.add_plugin(ShapePlugin);

    }

}

fn clear_correction_system(mut query: Query<&mut Movable>)
{

    for mut correction in query.iter_mut()
    {

        correction.axes.clear();

    }

}

fn update_movable_system(mut query: Query<(&Transform, &Movable, &mut Sepax)>)
{

    for (transform, _movable, mut sepax) in query.iter_mut()
    {

        let position = (transform.translation.x, transform.translation.y);

        let shape = sepax.shape_mut();
        shape.set_position(position);

    }

}

fn collision_system(mut movable: Query<(&mut Movable, &mut Sepax, &mut Transform)>, walls: Query<&Sepax, Without<Movable>>)
{

    for (mut correct, mut sepax, mut transform) in movable.iter_mut()
    {

        for wall in walls.iter()
        {

            let shape = sepax.shape_mut();
            let correction = sat_collision(wall.shape(), shape);

            let old_position = shape.position();
            let new_position = (old_position.0 + correction.0, old_position.1 + correction.1);

            shape.set_position(new_position);
            transform.translation.x = new_position.0;
            transform.translation.y = new_position.1;

            let length = f32::sqrt((correction.0 * correction.0) + (correction.1 * correction.1));
            correct.axes.push((correction.0 / length, correction.1 / length));

        }

    }

}