use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // Player's sprite size.
pub const ENEMIES_NUM: usize = 4;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_systems((spawn_player, spawn_camera, spawn_enemies))
        .add_systems((move_player, limit_player_movement))
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

// Update player transform every frame.
pub fn move_player(
    clock: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // Do not move non-existent player.
    let mut player_transform = match player_query.get_single_mut() {
        Ok(v) => v,
        Err(error) => {
            println!("Error when moving the player: {}", error);
            return;
        }
    };

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    player_transform.translation += direction * PLAYER_SPEED * clock.delta_seconds();
}

// Prevents player for going off the screen borders.
pub fn limit_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Do not move non-existent player.
    let mut player_transform = match player_query.get_single_mut() {
        Ok(v) => v,
        Err(error) => {
            println!("Error when limiting player movement: {}", error);
            return;
        }
    };
    let primary_window = window_query.get_single().unwrap();

    let player_half_size = PLAYER_SIZE / 2.0;
    let x_min = 0.0 + player_half_size;
    let x_max = primary_window.width() - player_half_size;
    let y_min = 0.0 + player_half_size;
    let y_max = primary_window.height() - player_half_size;

    let mut player_translation = player_transform.translation;
    if player_translation.x < x_min {
        player_translation.x = x_min;
    }
    if player_translation.x > x_max {
        player_translation.x = x_max;
    }
    if player_translation.y < y_min {
        player_translation.y = y_min;
    }
    if player_translation.y > y_max {
        player_translation.y = y_max;
    }

    player_transform.translation = player_translation;
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/Default/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let primary_window = window_query.get_single().unwrap();

    for _ in 0..ENEMIES_NUM {
        let x_pos: f32 = random::<f32>() * primary_window.width();
        let y_pos: f32 = random::<f32>() * primary_window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                texture: asset_server.load("sprites/Default/ball_red_large.png"),
                ..default()
            },
            Enemy {},
        ));
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
