use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_prototype_lyon::prelude::*;

use sepax2d::prelude::*;

pub mod plugin;
pub mod components;

use components::Sepax;

pub enum Convex
{

    Polygon(Polygon),
    Circle(Circle),
    AABB(AABB),
    Capsule(Capsule)

}

#[cfg(feature = "debug")]
pub fn spawn_debug(commands: &mut Commands, convex: Convex, fill: DrawMode)
{

    let shape = Sepax::as_shape_bundle(&convex, fill);

    let mut entity = commands.spawn();
    entity.insert(Sepax { convex }).insert_bundle(shape);

}

pub mod prelude
{

    pub use crate::Convex;

    pub use crate::plugin::SepaxPlugin;
    pub use crate::components::{Sepax, Movable};

}