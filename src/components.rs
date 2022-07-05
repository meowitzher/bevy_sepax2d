use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_prototype_lyon::{prelude::*, entity::ShapeBundle};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use sepax2d::Shape;

use crate::Convex;

/// A component encapsulating a shape for collision detection. 
/// A reference to the shape can be obtained using the [`shape`](Sepax::shape)
/// method without need for `match`ing the internal enum, although
/// `match` can be used when your behavior is dependent on the type of
/// shape.
#[derive(Component)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Sepax
{

    pub convex: Convex

}

/// A component which denotes that the entity is moving and colliding
/// with immovable entities. `axes` contains a list of the normalized collision
/// resolution vectors which point away from the immovable object that was
/// collided with. For example, if landing on flat ground, axes would contain
/// (0,1) on the next frame.
/// 
/// This list is cleared during the `PostUpdate` stage each frame when utilizing 
/// the plugin.
#[derive(Component)]
pub struct Movable
{

    pub axes: Vec<(f32, f32)>

}

/// A marker struct which tells the Sepax plugin not to perform collision checks on
/// the entity it is attached to. Collision checks can still be performed in custom
/// systems.
#[derive(Component)]
pub struct NoCollision;

impl Sepax
{

    /// A reference to the component's shape. As [`sat_overlap`](sepax2d::sat_overlap) and
    /// [`sat_collision`](sepax2d::sat_collision) take references as input, this will be
    /// the primary way to perform collision detection.
    pub fn shape(&self) -> &dyn Shape
    {

        match &self.convex
        {

            Convex::Polygon(poly) => poly,
            Convex::Circle(circle) => circle,
            Convex::AABB(aabb) => aabb,
            Convex::Capsule(capsule) => capsule

        }

    }

    /// A mutable reference to the shape. Mostly used so that the position can be updated.
    /// 
    /// When using the `"debug"` feature, you will need to update the rendering information
    /// if you mutate something else about the shape, such as size or vertices.
    pub fn shape_mut(&mut self) -> &mut dyn Shape
    {

        match &mut self.convex
        {

            Convex::Polygon(poly) => poly,
            Convex::Circle(circle) => circle,
            Convex::AABB(aabb) => aabb,
            Convex::Capsule(capsule) => capsule

        }

    }

    /// A convenience method for obtaining the shape information as a `Path`. This is
    /// particularly useful for changing collision shapes on the fly once the rendering
    /// bundle has already been inserted into the entity. Check out the platformer
    /// example to see this being done.
    #[cfg(feature = "debug")]
    pub fn shape_geometry(convex: &Convex) -> Path
    {

        match convex
        {

            Convex::Polygon(poly) =>
            {

                let mut builder = PathBuilder::new();
                
                if let Some((x, y)) = poly.vertices.first()
                {
                
                    builder.move_to(Vec2::new(*x, *y));

                    for (x, y) in poly.vertices.iter().cycle().skip(1).take(poly.vertices.len())
                    {

                        builder.line_to(Vec2::new(*x, *y));

                    }

                }

                builder.build()

            },
            Convex::Circle(circle) =>
            {
                
                ShapePath::build_as(&shapes::Circle { radius: circle.radius, center: Vec2::new(0.0, 0.0) })

            },
            Convex::AABB(aabb) =>
            {

                ShapePath::build_as(&shapes::Rectangle { extents: Vec2::new(aabb.width, aabb.height), origin: RectangleOrigin::BottomLeft })

            },
            Convex::Capsule(capsule) =>
            {

                let mut builder = PathBuilder::new();

                let arm = capsule.arm();
                let perp = capsule.perp();

                let angle = f32::atan2(perp.1, perp.0);

                builder.move_to(Vec2::new(arm.0 - perp.0, arm.1 - perp.1));
                builder.arc(Vec2::new(arm.0, arm.1), Vec2::new(capsule.radius, capsule.radius), std::f32::consts::PI, angle);
                builder.line_to(Vec2::new(-arm.0 + perp.0, -arm.1 + perp.1));
                builder.arc(Vec2::new(-arm.0, -arm.1), Vec2::new(capsule.radius, capsule.radius), std::f32::consts::PI, angle + std::f32::consts::PI);
                builder.line_to(Vec2::new(arm.0 - perp.0, arm.1 - perp.1));

                builder.build()

            }

        }

    }

    /// A convenience method for obtaining the components necessary to draw a given shape!
    /// Simple insert the return value into your entity as with any other bundle. Note that this
    /// does not insert the `Sepax` component itself, so that needs to be inserted as well if
    /// you want collisions to occur.
    /// 
    /// Requires "debug" feature.
    #[cfg(feature = "debug")]
    pub fn as_shape_bundle(convex: &Convex, fill: DrawMode) -> ShapeBundle
    {

        let position = match convex
        {

            Convex::Polygon(poly) => poly.position(),
            Convex::Circle(circle) => circle.position(),
            Convex::AABB(aabb) => aabb.position(),
            Convex::Capsule(capsule) => capsule.position(),

        };

        let shape = Sepax::shape_geometry(convex);

        GeometryBuilder::build_as(&shape, fill, Transform::from_xyz(position.0, position.1, 0.0))

    }

}