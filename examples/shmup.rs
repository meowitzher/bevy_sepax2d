use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::prelude::*;

use sepax2d::prelude::*;
use bevy_sepax2d::prelude::*;

const MARGIN: f32 = 10.0;

const SPEED: f32 = 250.0;

const PLAYER_RADIUS: f32 = 15.0;
const LASER_HALF: f32 = 8.0;
const LASER_RADIUS: f32 = 5.0;
const LASER_SPEED: f32 = 1000.0;

const ENEMY_SIZE: f32 = 20.0;
const ENEMY_SPEED: f32 = 250.0;

struct WindowSize
{

    width: f32,
    height: f32

}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Laser
{

    x: f32,
    y: f32

}

#[derive(Component)]
struct Enemy
{

    x: f32,
    y: f32

}

struct LastSpawn
{

    time: u64

}

fn main()
{

    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(WindowDescriptor
    {

        title: "Shmup Example".to_string(),
        width: 1024.0,
        height: 768.0,
        ..default()

    })
    .add_plugins(DefaultPlugins)
    .add_plugin(SepaxPlugin)
    .add_startup_system(setup_system)
    .add_startup_system(player_setup_system)
    .add_startup_system_to_stage(StartupStage::PostStartup, wall_setup_system)
    .add_system(player_movement_input_system)
    .add_system(player_shoot_input_system)
    .add_system(laser_velocity_system)
    .add_system(laser_despawn_system)
    .add_system(laser_hit_system)
    .add_system(enemy_spawn_system)
    .add_system(enemy_velocity_system)
    .add_system(enemy_collision_system.before(enemy_velocity_system))
    .add_system(game_over_system)
    .run();

}

fn setup_system(mut commands: Commands, mut windows: ResMut<Windows>, assets: Res<AssetServer>)
{

    let window = windows.get_primary_mut().unwrap();
    let window_size = WindowSize { width: window.width(), height: window.height() };
    commands.insert_resource(window_size);
    commands.insert_resource(LastSpawn { time: 0 });

    commands.spawn_bundle(Camera2dBundle::default());

    let font = assets.load("PolandCanInto.otf");
    let text_alignment = TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center };
    let text_style = TextStyle { font, font_size: 30.0, color: Color::rgba(0.8, 0.8, 0.8, 1.0) };

    commands.spawn_bundle(Text2dBundle
    {

        text: Text::from_section("WASD to move, Click to shoot", text_style.clone()).with_alignment(text_alignment),
        transform: Transform::from_xyz(0.0, 300.0, 0.0),
        ..default()

    });

}

fn player_setup_system(mut commands: Commands)
{

    let circle = Circle::new((0.0, 0.0), PLAYER_RADIUS);

    let player = DrawMode::Fill(FillMode::color(Color::rgba(0.2, 0.2, 1.0, 1.0)));
    let convex = Convex::Circle(circle);

    commands.spawn()
    .insert_bundle(Sepax::as_shape_bundle(&convex, player))
    .insert(Sepax { convex })
    .insert(Movable { axes: Vec::new() })
    .insert(Player);

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

}

fn player_movement_input_system(keyboard: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>, time: Res<Time>)
{

    if let Ok(mut transform) = query.get_single_mut()
    {

        if keyboard.pressed(KeyCode::A)
        {

            transform.translation.x -= SPEED * time.delta_seconds();

        }
        else if keyboard.pressed(KeyCode::D)
        {

            transform.translation.x += SPEED * time.delta_seconds();

        }

        if keyboard.pressed(KeyCode::W)
        {

            transform.translation.y += SPEED * time.delta_seconds();

        }
        else if keyboard.pressed(KeyCode::S)
        {

            transform.translation.y -= SPEED * time.delta_seconds();

        }

    }

}

fn player_shoot_input_system(mut commands: Commands, player: Query<&Transform, With<Player>>, windows: Res<Windows>, buttons: Res<Input<MouseButton>>, size: Res<WindowSize>)
{

    if let Ok(transform) = player.get_single()
    {

        let window = windows.get_primary().unwrap();

        if let Some(position) = window.cursor_position()
        {

            let position = (position.x - (size.width * 0.5), position.y - (size.height * 0.5));

            if buttons.just_pressed(MouseButton::Left)
            {

                let direction_vector = (position.0 - transform.translation.x, position.1 - transform.translation.y);

                let angle = f32::atan2(direction_vector.1, direction_vector.0);
                let normal = (f32::cos(angle), f32::sin(angle));

                let starting = (transform.translation.x + (PLAYER_RADIUS * normal.0), transform.translation.y + (PLAYER_RADIUS * normal.1));

                let convex = Convex::Capsule(Capsule::new(starting, (LASER_HALF * normal.0, LASER_HALF * normal.1), LASER_RADIUS));
                let laser = DrawMode::Fill(FillMode::color(Color::rgba(0.7, 0.7, 1.0, 1.0)));

                commands.spawn()
                .insert_bundle(Sepax::as_shape_bundle(&convex, laser))
                .insert(Sepax { convex })
                .insert(Movable { axes: Vec::new() })
                .insert(Laser { x: LASER_SPEED * normal.0, y: LASER_SPEED * normal.1 });

            }

        }

    }

}

fn laser_velocity_system(mut lasers: Query<(&mut Transform, &Laser)>, time: Res<Time>)
{

    for (mut transform, laser) in lasers.iter_mut()
    {

        transform.translation.x += laser.x * time.delta_seconds();
        transform.translation.y += laser.y * time.delta_seconds();

    }

}

fn laser_despawn_system(mut commands: Commands, query: Query<(Entity, &Movable), With<Laser>>)
{

    for (entity, correction) in query.iter()
    {

        //Collided with a non-Movable, therefore despawn
        if correction.axes.len() > 0
        {

            commands.entity(entity).despawn();
        
        }

    }

}

fn laser_hit_system(mut commands: Commands, lasers: Query<(Entity, &Sepax), (With<Laser>, Without<Enemy>)>, enemies: Query<(Entity, &Sepax), (With<Enemy>, Without<Laser>)>)
{

    for (laser, laser_sepax) in lasers.iter()
    {

        for (enemy, enemy_sepax) in enemies.iter()
        {

            if sat_overlap(enemy_sepax.shape(), laser_sepax.shape())
            {

                commands.entity(laser).despawn();
                commands.entity(enemy).despawn();

            }

        }

    }

}

fn spawn(seconds: u64) -> bool
{

    let spawn = 
        (seconds < 30 && seconds % 3 == 0) ||
        (seconds < 50 && seconds % 2 == 0) ||
        (seconds >= 50);

    spawn

}

fn enemy_spawn_system(mut commands: Commands, time: Res<Time>, size: Res<WindowSize>, mut last: ResMut<LastSpawn>)
{

    let time = time.time_since_startup().as_secs();

    if time > last.time && spawn(time)
    {

        last.time = time;

        let mut rng = thread_rng();
        let side: i32 = rng.gen_range(0..4);
        let spawn: f32 = rng.gen_range(-1.0..1.0);

        let (position, angle) = match side
        {

            //Top
            0 => 
            (
            
                (spawn * ((size.width * 0.5) - MARGIN),(size.height * 0.5) - MARGIN - ENEMY_SIZE),
                std::f32::consts::PI

            ),
            //Bottom
            1 => 
            (

                (spawn * ((size.width * 0.5) - MARGIN),-(size.height * 0.5) + MARGIN),
                0.0

            ),
            //Left
            2 =>
            (
            
                (-(size.width * 0.5) + MARGIN, spawn * ((size.height * 0.5) - MARGIN)),
                -(0.5 * std::f32::consts::PI)

            ),
            //Right
            _ => 
            (
            
                ((size.width * 0.5) - (2.0 * MARGIN), spawn * ((size.height * 0.5) - MARGIN - ENEMY_SIZE)),
                (0.5 * std::f32::consts::PI)
        
            )

        };

        let angle = rng.gen_range(angle..angle + std::f32::consts::PI);
        let velocity = (f32::cos(angle) * ENEMY_SPEED, f32::sin(angle) * ENEMY_SPEED);

        let convex = Convex::AABB(AABB::new(position, ENEMY_SIZE, ENEMY_SIZE));
        let enemy = DrawMode::Fill(FillMode::color(Color::rgba(1.0, 0.4, 0.4, 1.0)));

        commands.spawn()
        .insert_bundle(Sepax::as_shape_bundle(&convex, enemy))
        .insert(Sepax { convex })
        .insert(Movable { axes: Vec::new() })
        .insert(Enemy { x: velocity.0, y: velocity.1 });

    }

}

fn enemy_velocity_system(mut enemies: Query<(&mut Transform, &Enemy)>, time: Res<Time>)
{

    for (mut transform, enemy) in enemies.iter_mut()
    {

        transform.translation.x += enemy.x * time.delta_seconds();
        transform.translation.y += enemy.y * time.delta_seconds();

    }

}

fn enemy_collision_system(mut query: Query<(&Movable, &mut Enemy)>)
{

    for (correction, mut enemy) in query.iter_mut()
    {

        for (x, y) in correction.axes.iter()
        {

            let projection = (enemy.x * x) + (enemy.y * y);

            enemy.x -= 2.0 * projection * x;
            enemy.y -= 2.0 * projection * y;

        }

    }

}

fn game_over_system(mut commands: Commands, players: Query<(Entity, &Sepax), (With<Player>, Without<Enemy>)>, enemies: Query<(Entity, &Sepax), (With<Enemy>, Without<Player>)>, assets: Res<AssetServer>)
{

    if let Ok((player, player_sepax)) = players.get_single()
    {

        for (enemy, enemy_sepax) in enemies.iter()
        {

            if sat_overlap(enemy_sepax.shape(), player_sepax.shape())
            {

                commands.entity(player).despawn();
                commands.entity(enemy).despawn();

                let font = assets.load("PolandCanInto.otf");
                let text_alignment = TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center };
                let text_style = TextStyle { font, font_size: 30.0, color: Color::rgba(0.8, 0.8, 0.8, 1.0) };
            
                commands.spawn_bundle(Text2dBundle
                {
            
                    text: Text::from_section("Game Over!", text_style.clone()).with_alignment(text_alignment),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
            
                });

            }

        }

    }

}