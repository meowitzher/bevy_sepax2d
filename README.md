[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![Crates.io](https://img.shields.io/crates/v/bevy_sepax2d.svg)](https://crates.io/crates/bevy_sepax2d)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](./LICENSE)

# bevy_sepax2d
Plugins and helpful methods for using sepax2d with Bevy for 2d overlap detection and collision resolution. 

### Compatible Versions

| bevy | bevy_sepax2d |
|------|--------------|
| 0.12 | 0.5          |
| 0.9  | 0.4          |
| 0.8  | 0.2, 0.3     |
| 0.7  | 0.1          |

### Usage

Add the following to the `[dependencies]` section of your `Cargo.toml`:

```toml
sepax2d = "0.3"
bevy_sepax2d = "0.5"
```

There is an additional `debug` feature which can be used to render collision shapes to the screen.
This relies on Bevy's default features as well as [bevy_prototype_lyon](https://crates.io/crates/bevy_prototype_lyon)
for rendering. This can be enabled in your Cargo.toml:

```toml
bevy_sepax2d = { version = "0.3", features = ["debug"] }
```

To add a shape to your world, simply insert a `Sepax` struct into any entity.

```rust
let polygon = Polygon::from_vertices((0.0, 0.0), vec![(0.0, -25.0), (15.0, 15.0), (-15.0, 15.0)]);
let convex = Convex::Polygon(polygon);

commands.spawn(Sepax { convex });
```

`Sepax` has one field, `convex`: This is an instance of the `Convex` enum, which has possible values
for each shape supported by sepax2d: `Polygon`, `Circle`, `AABB`, `Parallelogram`, and `Capsule`. Each variant contains
an instance of the corresponding shape. Check the sepax2d 
[documentation](https://docs.rs/sepax2d/latest/sepax2d/index.html) for information about each one.

The underlying shape can be conveniently accessed through the `shape` and `shape_mut` methods, which provide
easy access to references to the underlying shapes without need to match the enum.

```rust
fn bullet_system
(
    mut bullets: Query<(&Bullet, &Sepax)>,
    targets: Query<&Sepax, Without<Bullet>>
)
{
    for (_b, bullet) in bullets.iter()
    {
        for target in targets.iter()
        {
            if sat_overlap(wall.shape(), bullet.shape())
            {
                //Bullet hit target, now react appropriately
            }
        }
    }
}
```

If included, the `SepaxPlugin` provides the following basic features, which occur during Bevy's
`PostUpdate` stage:

* Resets the collision information from the previous frame
* Updates the location of any `Sepax` component attached to a 
[`Transform`](https://docs.rs/bevy/latest/bevy/prelude/struct.Transform.html#impl-Default)
* Provides inelastic collision between entities with a `Sepax` shape which are tagged 
`Movable` and those that are not movable.

These systems are public, so you may include them manually if you do not want all of them.
This is likely to happen when you want to introduce finer control over which objects collide
with which, but still want to reset collision data and update locations. Or, you may want to
collide and update locations, but want to maintain old collision data until it has been processed.

The `Movable` component signifies that an entity is dynamic. By contrast, the absence of a `Movable`
component denotes a static object which is treated like a "wall" that movable entities should collide
with. Use the `NoCollision` marker component on either a `Movable` or non-`Movable` entity to exclude
it from the collision process.

The `Movable` struct contains a list of normalized collision resolution vectors from the previous frame during the
`Update` stage for you to react to in your code. These vectors represent the direction AWAY from the
object that was collided with. For example, the following code zeroes out the y-component
of an entity's velocity when it lands on or hits the bottom of a platform:

```rust
fn velocity_correction_system(mut query: Query<(&mut Velocity, &Movable)>)
{
    for (mut velocity, correction) in query.iter_mut()
    {
        for (_x, y) in correction.axes.iter()
        {
            if y.abs() > f32::EPSILON && velocity.y.is_sign_positive() != y.is_sign_positive()
            {
                velocity.y = 0.0;
            }
        }
    }
}
```

### Debug Rendering

If you enable the `debug` feature, then you can render your shapes with the help of bevy_prototype_lyon.
The following convenience methods help with rendering:

* `Sepax::as_shape_bundle` will take in a reference to a `Sepax` struct and a 
[`DrawMode`](https://docs.rs/bevy_prototype_lyon/latest/bevy_prototype_lyon/draw/enum.DrawMode.html)
and return a `ShapeBundle` to be added to an entity.
* `Sepax::shape_geometry` will take in a reference to a `Sepax` struct and return a
[`Path`](https://docs.rs/bevy_prototype_lyon/latest/bevy_prototype_lyon/path/index.html) representing the
given shape. Use this in conjunction with 
[`ShapePath`](https://docs.rs/bevy_prototype_lyon/latest/bevy_prototype_lyon/path/struct.ShapePath.html)
to change the rendered shape at runtime. Check out the platformer example to see this in action!

```rust
let circle = Circle::new((0.0, 0.0), 15.0);
let convex = Convex::Circle(circle);

let player = DrawMode::Fill(FillMode::color(Color::WHITE));

commands.spawn(Sepax::as_shape_bundle(&convex, player))
.insert(Sepax { convex })
.insert(Movable { axes: Vec::new() });
```

### Features
`debug` - Enables rendering of shapes.

`serde` - Enables (De)Serialization of Convex and Sepax types for easy loading.

### Examples
The repository includes two example applications showcasing a basic platformer (which only uses
the basic plugin), and a shmup which demonstrates some custom systems.

```sh
cargo run --features="debug" --example platformer

cargo run --features="debug" --example shmup
```

### Contribution
Please feel free to suggest additional features, bug fixes, or optimizations. Thanks!