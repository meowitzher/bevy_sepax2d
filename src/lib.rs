//! [![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
//! [![Crates.io](https://img.shields.io/crates/v/bevy_sepax2d.svg)](https://crates.io/crates/bevy_sepax2d)
//! [![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](./LICENSE)
//!
//! # bevy_sepax2d
//! Plugins and helpful methods for using sepax2d with Bevy for 2d overlap detection and collision resolution. 
//!
//! ### Compatible Versions
//!
//! |bevy|bevy_sepax2d|
//! |---|---|
//! |0.9|0.4|
//! |0.8|0.2, 0.3|
//! |0.7|0.1|
//!
//! ### Usage
//!
//! Add the following to the `[dependencies]` section of your `Cargo.toml`:
//!
//! ```toml
//! sepax2d = "0.3"
//! bevy_sepax2d = "0.3"
//! ```
//! 
//! There is an additional `debug` feature which can be used to render collision shapes to the screen.
//! This relies on Bevy's default features as well as [bevy_prototype_lyon](https://crates.io/crates/bevy_prototype_lyon)
//! for rendering. This can be enabled in your Cargo.toml:
//! 
//! ```toml
//! bevy_sepax2d = { version = "0.3", features = ["debug"] }
//! ```
//! 
//! To add a shape to your world, simply insert a [`Sepax`](components::Sepax) struct into any entity.
//! 
//! ```rust,no_run
//! use bevy::prelude::*;
//! use sepax2d::prelude::*;
//! use bevy_sepax2d::prelude::*;
//! 
//! fn spawn_system(mut commands: Commands)
//! {
//! 
//!     let polygon = Polygon::from_vertices((0.0, 0.0), vec![(0.0, -25.0), (15.0, 15.0), (-15.0, 15.0)]);
//!     let convex = Convex::Polygon(polygon);
//! 
//!     commands.spawn(Sepax { convex });
//! 
//! }
//! ```
//! 
//! [`Sepax`](components::Sepax) has one field, `convex`: This is an instance of the [`Convex`](Convex) enum, which has possible values
//! for each shape supported by sepax2d: [`Polygon`](sepax2d::polygon::Polygon), [`Circle`](sepax2d::circle::Circle),
//! [`AABB`](sepax2d::aabb::AABB), [`Parallelogram`](sepax2d::parallelogram::Parallelogram), and [`Capsule`](sepax2d::capsule::Capsule). Each variant contains
//! an instance of the corresponding shape. 
//! 
//! The underlying shape can be conveniently accessed through the [`shape`](components::Sepax::shape) and 
//! [`shape_mut`](components::Sepax::shape_mut) methods, which provide easy access to references to the
//! underlying shapes without need to match the enum.
//! 
//! ```rust,no_run
//! use bevy::prelude::*;
//! use sepax2d::prelude::*;
//! use bevy_sepax2d::prelude::*;
//! 
//! #[derive(Component)]
//! struct Bullet;
//! 
//! fn bullet_system
//! (
//!     mut bullets: Query<(&Bullet, &Sepax)>,
//!     targets: Query<&Sepax, Without<Bullet>>
//! )
//! {
//!     for (_b, bullet) in bullets.iter()
//!    {
//!         for target in targets.iter()
//!         {
//!             if sat_overlap(target.shape(), bullet.shape())
//!             {
//!                 //Bullet hit target, now react appropriately
//!             }
//!         }
//!     }
//! }
//! ```
//! 
//! ### Features
//! 
//! `debug` - Enables rendering of shapes.
//! 
//! `serde` - Enables (De)Serialization of Convex and Sepax types for easy loading.

#[cfg(feature = "debug")]
use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_prototype_lyon::prelude::*;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use sepax2d::prelude::*;

pub mod plugin;
pub mod components;

#[cfg(feature = "debug")]
use components::Sepax;

/// An enum for the different types of shapes supported by sepax2d.
/// For most use cases, you will store a `Convex` inside of a
/// [`Sepax`](components::Sepax)
/// and will be able to use the
/// [`shape`](components::Sepax::shape)
/// method to avoid `match`ing the enum directly. For use cases where your behavior
/// depends on the type of shape, then you will need to use `match`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Convex
{

    Polygon(Polygon),
    Circle(Circle),
    AABB(AABB),
    Parallelogram(Parallelogram),
    Capsule(Capsule)

}

/// A quick-and-dirty convenience method for spawning immovable shapes. Does not return
/// any way to access the created entity, so it is only recommended for use in small
/// projects or prototypes. Use 
/// [as_shape_bundle](components::shapes::Sepax::as_shape_bundle)
/// instead if you need to further modify the created entities.
/// 
/// Requires the "debug" feature.
#[cfg(feature = "debug")]
pub fn spawn_debug(commands: &mut Commands, convex: Convex, fill: Fill)
{

    let shape = Sepax::as_shape_bundle(&convex);

    commands.spawn((Sepax { convex },
                    fill
    ))
    .insert(shape);

}

pub mod prelude
{

    pub use crate::Convex;

    pub use crate::plugin::SepaxPlugin;
    pub use crate::components::{Sepax, NoCollision, Movable};

}