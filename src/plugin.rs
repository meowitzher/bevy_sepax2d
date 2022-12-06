use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_prototype_lyon::prelude::*;

use sepax2d::prelude::*;

use crate::components::{Movable, NoCollision, Sepax};

/// A simple plugin which adds some basic functionality to your Bevy app!
/// 
/// * Resets the collision information from the previous frame ([`clear_correction_system`](clear_correction_system))
/// * Updates the location of any `Sepax` component attached to a 
/// [`Transform`](https://docs.rs/bevy/latest/bevy/prelude/struct.Transform.html#impl-Default)
/// ([`update_movable_system`](update_movable_system))
/// * Provides inelastic collision between entities with a `Sepax` shape which are tagged 
/// `Movable` and those that are not movable. ([`collision_system`](collision_system))
///
/// Each of the above systems is public for you to manually add to your app if you want some but not all.
pub struct SepaxPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum SepaxSystems
{

    Clear,
    Update,
    Collision

}

impl Plugin for SepaxPlugin
{

    fn build(&self, app: &mut App)
    {

        app
        .add_system_to_stage
        (

            CoreStage::PostUpdate,
            clear_correction_system
            .label(SepaxSystems::Clear)

        )
        .add_system_to_stage
        (

            CoreStage::PostUpdate,
            update_movable_system.after(clear_correction_system)
            .label(SepaxSystems::Update)

        )
        .add_system_to_stage
        (
            
            CoreStage::PostUpdate, 
            collision_system
            .label(SepaxSystems::Collision)
            .after(SepaxSystems::Update)
            .before(bevy::transform::transform_propagate_system)

        );

        #[cfg(feature = "debug")]
        app.add_plugin(ShapePlugin);

    }

}

/// [`Movable`](crate::components::Movable) components store a list of axes
/// that were used for collision resolution on the previous frame. This system
/// resets that list each frame before the collision system generates new data.
pub fn clear_correction_system(mut query: Query<&mut Movable>)
{

    for mut correction in query.iter_mut()
    {

        correction.axes.clear();

    }

}

/// Updates the position information contained inside of [`Sepax`](crate::components::Sepax)
/// components to match the entity's translation in the world. This is necessary because
/// sepax2d is not a Bevy-centric crate, so it does not use Transforms natively.
pub fn update_movable_system(mut query: Query<(&Transform, &Movable, &mut Sepax)>)
{

    for (transform, _movable, mut sepax) in query.iter_mut()
    {

        let position = (transform.translation.x, transform.translation.y);

        let shape = sepax.shape_mut();
        shape.set_position(position);

    }

}

/// Performs inelastic collisions between all [`Movable`](crate::components::Movable) and all immovable
/// entities. If there is a collision, the normalized axis of resolution is stored inside the `Movable`
/// component for use in your app. This points away from the immovable object. For example, if you are 
/// making a platformer and want to check if the player has landed on something, you would check for
/// axes with a positive y component. 
pub fn collision_system(mut movable: Query<(&mut Movable, &mut Sepax, &mut Transform), Without<NoCollision>>, walls: Query<&Sepax, (Without<Movable>, Without<NoCollision>)>)
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

            if length > f32::EPSILON
            {

                correct.axes.push((correction.0 / length, correction.1 / length));

            }

        }

    }

}