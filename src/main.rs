use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // Player's sprite size.
pub const ENEMIES_NUM: usize = 4;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 400.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_systems((spawn_player, spawn_camera, spawn_enemies))
        .add_systems((
            move_player,
            limit_player_movement,
            move_enemy,
            update_enemy_direction,
            limit_enemy_movement,
            check_collision,
        ))
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec3,
}

// Update player transform every frame.
pub fn move_player(
    clock: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // Do not move non-existent player.
    let mut player_transform = match player_query.get_single_mut() {
        Ok(v) => v,
        Err(_error) => {
            // println!("Error when moving the player: {}", error);
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
        Err(_error) => {
            // println!("Error when limiting player movement: {}", error);
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

pub fn move_enemy(mut enemy_query: Query<(&mut Transform, &Enemy)>, clock: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        transform.translation += enemy.direction * ENEMY_SPEED * clock.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let primary_window = window_query.get_single().unwrap();

    let enemy_half_size = PLAYER_SIZE / 2.0;
    let x_min = 0.0 + enemy_half_size;
    let x_max = primary_window.width() - enemy_half_size;
    let y_min = 0.0 + enemy_half_size;
    let y_max = primary_window.height() - enemy_half_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;

        let enemy_translation = transform.translation;
        if enemy_translation.x < x_min || enemy_translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if enemy_translation.y < y_min || enemy_translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if !direction_changed {
            continue;
        }

        // Play sound if direction did change.
        let bounce_sound1 = asset_server.load("audio/impact/footstep_concrete_000.ogg");
        let bounce_sound2 = asset_server.load("audio/impact/footstep_concrete_001.ogg");

        let sound_effect = if random::<f32>() > 0.5 {
            bounce_sound1
        } else {
            bounce_sound2
        };

        audio.play(sound_effect);
    }
}

pub fn limit_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = window_query.get_single().unwrap();

    let enemy_half_size = PLAYER_SIZE / 2.0;
    let x_min = 0.0 + enemy_half_size;
    let x_max = primary_window.width() - enemy_half_size;
    let y_min = 0.0 + enemy_half_size;
    let y_max = primary_window.height() - enemy_half_size;

    for mut transform in enemy_query.iter_mut() {
        let mut enemy_translation = transform.translation;
        if enemy_translation.x < x_min {
            enemy_translation.x = x_min;
        }
        if enemy_translation.x > x_max {
            enemy_translation.x = x_max;
        }
        if enemy_translation.y < y_min {
            enemy_translation.y = y_min;
        }
        if enemy_translation.y > y_max {
            enemy_translation.y = y_max;
        }

        transform.translation = enemy_translation;
    }
}

// Checks if the player and enemy collide with each other.
pub fn check_collision(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let (player_entity, player_transform) = match player_query.get_single_mut() {
        Ok(v) => v,
        Err(_error) => {
            // println!("Error when checking collision of player: {}", error);
            return;
        }
    };

    let player_radius = PLAYER_SIZE / 2.0;
    let player_translation = player_transform.translation;

    for enemy_transform in enemy_query.iter() {
        let enemy_radius = ENEMY_SIZE / 2.0;
        let enemy_translation = enemy_transform.translation;

        let distance = f32::sqrt(
            (player_translation.x - enemy_translation.x)
                * (player_translation.x - enemy_translation.x)
                + (player_translation.y - enemy_translation.y)
                    * (player_translation.y - enemy_translation.y),
        );
        if distance < player_radius + enemy_radius {
            // Play the sound and despawn player.
            let sound_effect = asset_server.load("audio/scifi/explosionCrunch_000.ogg");
            audio.play(sound_effect);

            commands.entity(player_entity).despawn();
        }
    }
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
            Enemy {
                direction: Vec3::new(random::<f32>(), random::<f32>(), 0.0).normalize(),
            },
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
