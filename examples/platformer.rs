use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use sepax2d::prelude::*;
use bevy_sepax2d::prelude::*;

const MARGIN: f32 = 10.0;

const GRAVITY: f32 = 500.0;
const TERMINAL: f32 = 2000.0;
const SPEED: f32 = 200.0;
const JUMP_SPEED: f32 = 500.0;

#[derive(Resource)]
struct WindowSize
{

    width: f32,
    height: f32

}

#[derive(Resource)]
struct PlayerColliders
{

    polygon: sepax2d::polygon::Polygon,
    circle: sepax2d::circle::Circle,
    aabb: sepax2d::aabb::AABB,
    gram: sepax2d::parallelogram::Parallelogram,
    capsule: sepax2d::capsule::Capsule,
    index: usize

}

impl PlayerColliders
{

    fn next(&mut self) -> Sepax
    {

        self.index = (self.index + 1) % 5;

        match self.index
        {

            0 => Sepax { convex: Convex::Polygon(self.polygon.clone()) },
            1 => Sepax { convex: Convex::Circle(self.circle) },
            2 => Sepax { convex: Convex::AABB(self.aabb) },
            3 => Sepax { convex: Convex::Parallelogram(self.gram) },
            _ => Sepax { convex: Convex::Capsule(self.capsule) }

        }

    }

}

#[derive(Component)]
struct Velocity
{

    x: f32,
    y: f32

}

fn main()
{

    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugins(DefaultPlugins.set
    (

        WindowPlugin
        {

            window: WindowDescriptor
            {
        
                title: "Platformer Example".to_string(),
                width: 1024.0,
                height: 768.0,
                ..default()
        
            },
            ..default()

        }

    ))
    .add_plugin(SepaxPlugin)
    .add_startup_system(setup_system)
    .add_startup_system(player_setup_system)
    .add_startup_system_to_stage(StartupStage::PostStartup, wall_setup_system)
    .add_system_to_stage(CoreStage::PreUpdate, velocity_correction_system)
    .add_system(player_movement_input_system.before(gravity_system))
    .add_system(player_collider_system)
    .add_system(gravity_system)
    .add_system(velocity_system)
    .run();

}

fn setup_system(mut commands: Commands, mut windows: ResMut<Windows>, assets: Res<AssetServer>)
{

    let window = windows.get_primary_mut().unwrap();
    let window_size = WindowSize { width: window.width(), height: window.height() };
    commands.insert_resource(window_size);

    commands.spawn(Camera2dBundle::default());

    let font = assets.load("PolandCanInto.otf");
    let text_alignment = TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center };
    let text_style = TextStyle { font, font_size: 30.0, color: Color::rgba(0.8, 0.8, 0.8, 1.0) };

    commands.spawn(Text2dBundle
    {

        text: Text::from_section("A and D to move, Space to jump \n \n W to change colliders", text_style.clone()).with_alignment(text_alignment),
        transform: Transform::from_xyz(0.0, 300.0, 0.0),
        ..default()

    });

}

fn player_setup_system(mut commands: Commands)
{

    let capsule = Capsule::new((0.0, 0.0), (0.0, 20.0), 15.0);
    let aabb = AABB::new((0.0, 0.0), 20.0, 50.0);
    let gram = Parallelogram::new((0.0, 0.0), (20.0, 10.0), (40.0, 50.0));
    let circle = Circle::new((0.0, 0.0), 25.0);
    let polygon = Polygon::from_vertices((0.0, 0.0), vec![(0.0, -25.0), (15.0, 15.0), (-15.0, 15.0)]);

    let player = DrawMode::Fill(FillMode::color(Color::rgba(0.4, 0.4, 1.0, 1.0)));

    let convex = Convex::Polygon(polygon.clone());

    commands.spawn(Sepax::as_shape_bundle(&convex, player))
    .insert(Sepax { convex })
    .insert(Movable { axes: Vec::new() })
    .insert(Velocity { x: 0.0, y: 0.0 });

    commands.insert_resource(PlayerColliders { polygon: polygon, circle, aabb, gram, capsule, index: 0 });

}

fn wall_setup_system(mut commands: Commands, size: Res<WindowSize>)
{

    let walls = DrawMode::Fill(FillMode::color(Color::rgba(0.8, 0.8, 0.8, 1.0)));

    let half_width = size.width * 0.5;
    let half_height = size.height * 0.5;

    //Outer Walls
    bevy_sepax2d::spawn_debug(&mut commands, Convex::AABB(AABB::new((-half_width - MARGIN, -half_height - MARGIN), 2.0 * MARGIN, size.height + (2.0 * MARGIN))), walls);
    bevy_sepax2d::spawn_debug(&mut commands, Convex::AABB(AABB::new((half_width - MARGIN, -half_height - MARGIN), 2.0 * MARGIN, size.height + (2.0 * MARGIN))), walls);
    bevy_sepax2d::spawn_debug(&mut commands, Convex::AABB(AABB::new((-half_width - MARGIN, -half_height - MARGIN), size.width + (2.0 * MARGIN), 2.0 * MARGIN)), walls);
    bevy_sepax2d::spawn_debug(&mut commands, Convex::AABB(AABB::new((-half_width - MARGIN, half_height - MARGIN), size.width + (2.0 * MARGIN), 2.0 * MARGIN)), walls);
    
    //Platforms
    bevy_sepax2d::spawn_debug(&mut commands, Convex::AABB(AABB::new((-half_width - MARGIN, -300.0), 400.0, 30.0)), walls);
    bevy_sepax2d::spawn_debug(&mut commands, Convex::AABB(AABB::new((150.0, 100.0), 300.0, 30.0)), walls);
    bevy_sepax2d::spawn_debug(&mut commands, Convex::AABB(AABB::new((-175.0, 50.0), 50.0, 100.0)), walls);

    bevy_sepax2d::spawn_debug(&mut commands, Convex::Circle(Circle::new((0.0, -100.0), 50.0)), walls);

    bevy_sepax2d::spawn_debug(&mut commands, Convex::Capsule(Capsule::new((200.0, -150.0), (25.0, 20.0), 20.0)), walls);

    bevy_sepax2d::spawn_debug(&mut commands, Convex::Polygon(Polygon::from_vertices((-450.0, -100.0), vec![(0.0, 0.0), (100.0, 0.0), (150.0, 100.0), (50.0, 150.0), (-50.0, 100.0)])), walls);

}

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

fn gravity_system(mut query: Query<(&Movable, &mut Velocity)>, time: Res<Time>)
{

    for (_move, mut velocity) in query.iter_mut()
    {

        velocity.y -= GRAVITY * time.delta_seconds();
        velocity.y = f32::max(velocity.y, -TERMINAL);

    }

}

fn velocity_system(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>)
{

    for (velocity, mut transform) in query.iter_mut()
    {

        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();

    }

}

fn player_movement_input_system(keyboard: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Movable>>)
{

    if let Ok(mut velocity) = query.get_single_mut()
    {

        if keyboard.pressed(KeyCode::A)
        {

            velocity.x = -SPEED;

        }
        else if keyboard.pressed(KeyCode::D)
        {

            velocity.x = SPEED;

        }
        else 
        {

            velocity.x = 0.0;

        }

        if keyboard.just_pressed(KeyCode::Space)
        {

            velocity.y = JUMP_SPEED;

        }

    }

}

fn player_collider_system(keyboard: Res<Input<KeyCode>>, mut query: Query<(&mut Path, &mut Sepax), With<Movable>>, mut colliders: ResMut<PlayerColliders>)
{

    if let Ok((mut path, mut sepax)) = query.get_single_mut()
    {

        if keyboard.just_pressed(KeyCode::W)
        {

            *sepax = colliders.next();
            *path = ShapePath::build_as(&Sepax::shape_geometry(&sepax.convex));

        }

    }

}