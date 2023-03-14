use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.05, 0.05, 0.05);
const GRAVITY: Vec2 = Vec2::new(0.0,-1000.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
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
    commands.spawn(SpriteBundle {
        texture: asset_server.load("robot.png"),
        ..default()
    });
}

fn check_for_collisions(
    mut commands: Commands,
    mut player_query: Query<(&mut Velocity, &Transform), With<Player>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
) {

}