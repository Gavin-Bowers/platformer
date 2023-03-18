use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.05, 0.05, 0.05);
const GRAVITY: Vec2 = Vec2::new(0.0,-1.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_systems((
            apply_gravity,
            check_for_collisions,
            apply_velocity
        ).chain())
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Platform;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    //Player
    commands.spawn((SpriteBundle {
        texture: asset_server.load("robot.png"),
        transform: Transform::from_translation(Vec3::new(0.0,0.0,0.0)),
        sprite: Sprite {/*custom_size: Some(Vec2::new(100.0,200.0)),*/ ..default()},
        ..default()
        },
        Player,
        Velocity(Vec2::new(0.0,0.0)),
    ));

    //platform
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.,-150.,0.),
                scale: Vec3::new(100., 100., 1.0),
                ..default()
            },
            ..default()
        },
        Collider,
    ));
}

//Order is gravity, then collision, then movement
fn apply_gravity(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in &mut query {
        velocity.0 += GRAVITY;
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in &mut query {
        transform.translation.x += velocity.0.x * TIME_STEP;
        transform.translation.y += velocity.0.y * TIME_STEP;
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut player_query: Query<(&mut Velocity, &Transform), With<Player>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
) {
    let (mut player_velocity, player_transform) = player_query.single_mut();
    let player_size = player_transform.scale.truncate();
    
    //Check collision with obstacles
    for (collider_entity, transform) in &collider_query {
        let collision = collide(
            player_transform.translation,
            player_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            match collision {
                Collision::Left => if player_velocity.0.y < 0.0 {player_velocity.0.y = 0.0},
                Collision::Right => if player_velocity.0.y < 0.0 {player_velocity.0.y = 0.0},
                Collision::Top => (),
                Collision::Bottom => (),
                Collision::Inside => ()
            }
        }
    }
}