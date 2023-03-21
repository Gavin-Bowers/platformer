use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision}, 
    window::PresentMode,
};
use bevy_prototype_debug_lines::*;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.05, 0.05, 0.05);
const GRAVITY: f32 = -12.0;
const SCREEN_WIDTH: f32 = 800.;
const SCREEN_HEIGHT: f32 = 600.;
const SPAWN_POINT: Vec3 = Vec3::new(0.0,60.0,0.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2D Platformer".into(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: false,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(DebugLinesPlugin::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_systems((
            move_player,
            check_for_collisions,
            apply_velocity,
            zone_transition
        ).chain())
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Player {
    jump: bool,
    dash: bool,
    grounded: bool,
    dead: bool,
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Hazard;

#[derive(Component)]
struct CurrentZone(Vec2);

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    //Player
    commands.spawn((SpriteBundle {
        texture: asset_server.load("robot.png"),
        transform: Transform {
            translation: SPAWN_POINT,
            scale: Vec3::new(30.0,40.0,1.0),
            ..Default::default()
        },
        sprite: Sprite {custom_size: Some(Vec2::new(1.0,1.0)), ..default()},
        ..default()
        },
        Player {jump: false, dash: false, grounded:false, dead: false},
        Velocity(Vec2::new(0.0,0.0)),
    ));

    //platforms
    spawn_platform(&mut commands, Vec3::new(0.,0.,0.0), Vec3::new(200., 10., 1.0));
    spawn_platform(&mut commands, Vec3::new(400.,50.,0.0), Vec3::new(100., 10., 1.0));
    spawn_hazard(&mut commands, Vec3::new(0.,0.,0.0), Vec3::new(1000., 10., 1.0));
}

///////////////////////////////////////////////////////////////////////////////////
/// Objects

fn spawn_platform(
    mut commands: &mut Commands,
    position: Vec3,
    scale: Vec3, ) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GRAY,
                ..default()
            },
            transform: Transform {
                translation: position,
                scale: scale,
                ..default()
            },
            ..default()
        },
        Collider,
    ));
}

fn spawn_hazard(
    mut commands: &mut Commands,
    position: Vec3,
    scale: Vec3, ) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE_RED,
                ..default()
            },
            transform: Transform {
                translation: position,
                scale: scale,
                ..default()
            },
            ..default()
        },
        Collider, Hazard,
    ));
}

///////////////////////////////////////////////////////////////////////////////////
/// Physics

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.0.x * TIME_STEP;
        transform.translation.y += velocity.0.y * TIME_STEP;
    }
}

fn check_for_collisions(
    mut player_query: Query<(&mut Velocity, &Transform), With<Player>>,
    mut player_data_query: Query<&mut Player>,
    collider_query: Query<(Entity, &Transform, Option<&Hazard>), With<Collider>>,
    lines: ResMut<DebugLines>,
) {
    let (mut player_velocity, player_transform) = player_query.single_mut();
    /*draw_debug_rect(
        lines,
        player_transform.as_ref()
    );*/

    let mut any_collision: bool = false;
    
    //Check collision with obstacles
    for (_collider_entity, transform, maybe_hazard) in &collider_query {
        let collision = collide(
            player_transform.translation,
            player_transform.scale.truncate(),
            transform.translation,
            transform.scale.truncate(),
        );
        
        //The collision prevents the player from moving into objects in the direction of motion
        if let Some(collision) = collision {
            if let Some(_) = maybe_hazard {
                // You die
                player_data_query.single_mut().dead = true;
            }
            let mut on_ground: bool = false;
            any_collision = true;
            match collision {
                Collision::Left =>   if player_velocity.0.x > 0.0 {player_velocity.0.x = 0.0},
                Collision::Right =>  if player_velocity.0.x < 0.0 {player_velocity.0.x = 0.0},
                Collision::Top =>    on_ground = true,
                Collision::Bottom => if player_velocity.0.y > 0.0 {player_velocity.0.y = 0.0},
                Collision::Inside => (),
            }
            
            if on_ground {
                if player_velocity.0.y < 0.0 {player_velocity.0.y = 0.0};
                player_data_query.single_mut().grounded = true;
                player_data_query.single_mut().dash = true;
            } else {
                player_data_query.single_mut().jump = false;
                player_data_query.single_mut().grounded = false;
            }
        }
    }
    if !any_collision { player_data_query.single_mut().grounded = false; }
}

///////////////////////////////////////////////////////////////////////////////////
/// Debugging
/*
fn draw_debug_rect(mut lines: ResMut<DebugLines>, transform: &Transform) {
    let corners = get_sprite_corners(transform);
    let duration = 0.0;     // Duration of 0 will show the line for 1 frame.
    lines.line(corners[0], corners[1], duration);
    lines.line(corners[1], corners[3], duration);
    lines.line(corners[3], corners[2], duration);
    lines.line(corners[2], corners[0], duration);
}

fn get_sprite_corners(transform: &Transform) -> [Vec3; 4] {
    let half_width = 0.5 * transform.scale.x;
    let half_height = 0.5 * transform.scale.y;
    let position = transform.translation;

    let top_left = position + Vec3::new(-half_width, half_height, 0.0);
    let top_right = position + Vec3::new(half_width, half_height, 0.0);
    let bottom_left = position + Vec3::new(-half_width, -half_height, 0.0);
    let bottom_right = position + Vec3::new(half_width, -half_height, 0.0);

    [top_left, top_right, bottom_left, bottom_right]
}
*/
///////////////////////////////////////////////////////////////////////////////////
/// Controls

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
    mut player_data_query: Query<&mut Player>,
) {
    let mut player_velocity = query.single_mut();
    let mut direction_key_pressed: bool = false; //Determines whether a dash occurs when holding x

    //Left-right motion
    let mut direction: i8 = 0;
    if keyboard_input.pressed(KeyCode::Left) { direction -= 1; direction_key_pressed = true;}
    if keyboard_input.pressed(KeyCode::Right) { direction += 1; direction_key_pressed = true;}

    //Up-down motion
    let mut y_direction: i8 = 0;
    if keyboard_input.pressed(KeyCode::Down) { y_direction -= 1; direction_key_pressed = true;}
    if keyboard_input.pressed(KeyCode::Up) { y_direction += 1; direction_key_pressed = true;}

    //Grounded movement:
    if player_data_query.single_mut().grounded {
        if direction == -1 { 
            if player_velocity.0.x > -200.0 { player_velocity.0.x += -12.0 }
            if player_velocity.0.x > 0.0 { player_velocity.0.x -= 35.0 }
        }
        else if direction == 0 {
            if      player_velocity.0.x > 20.0  { player_velocity.0.x -= 15.0} 
            else if player_velocity.0.x < -20.0 { player_velocity.0.x += 15.0}
            else                               { player_velocity.0.x =  0.0} 
        }
        else if direction == 1 {
            if player_velocity.0.x < 200.0 { player_velocity.0.x += 12.0 }
            if player_velocity.0.x < 0.0 { player_velocity.0.x += 35.0 }
        }

        if keyboard_input.pressed(KeyCode::Z) {
            player_velocity.0.y = 300.0;
            player_data_query.single_mut().grounded = false;
        }

    } else { //Aerial movement
        player_velocity.0.y += GRAVITY;
        match direction {
            -1 => 
                if player_velocity.0.x > -200.0 { player_velocity.0.x += -12.0 },
            0 =>
                if      player_velocity.0.x > 2.0  { player_velocity.0.x -= 1.0} 
                else if player_velocity.0.x < -2.0 { player_velocity.0.x += 1.0}
                else                               { player_velocity.0.x =  0.0} ,
            1 => 
                if player_velocity.0.x < 200.0 { player_velocity.0.x += 12.0 },
            _ => (),
        }

        if  keyboard_input.pressed(KeyCode::X) &&
            player_data_query.single_mut().dash &&
            direction_key_pressed {
                player_velocity.0.x = 350.0 * direction as f32;
                player_velocity.0.y = 350.0 * y_direction as f32;
                player_data_query.single_mut().dash = false;
        }
    }
    //Ignoring climbing for now because it's a little harder to implement
}

fn zone_transition(
    mut query: Query<&mut Transform, With<Player>>,
    mut player_data_query: Query<&mut Player>,
) {
    if  query.single_mut().translation.x > SCREEN_WIDTH / 2.0 { query.single_mut().translation.x = SCREEN_WIDTH / -2.0 }
    if  query.single_mut().translation.x < SCREEN_WIDTH / -2.0 { query.single_mut().translation.x = SCREEN_WIDTH / 2.0 }
    if  query.single_mut().translation.y > SCREEN_HEIGHT / 2.0 { query.single_mut().translation.y = SCREEN_HEIGHT / -2.0 }
    if  query.single_mut().translation.y < SCREEN_HEIGHT / -2.0 { query.single_mut().translation.y = SCREEN_HEIGHT / 2.0 }

}