use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_prototype_lyon::{prelude::*, entity::ShapeBundle};

use sepax2d::Shape;

use crate::Convex;

#[derive(Component)]
pub struct Sepax
{

    pub convex: Convex

}

#[derive(Component)]
pub struct Movable
{

    pub axes: Vec<(f32, f32)>

}

impl Sepax
{

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

                let position = capsule.position;
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