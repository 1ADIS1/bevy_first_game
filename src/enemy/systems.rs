use super::components::*;
use super::resources::*;
use super::{ENEMIES_NUM, ENEMY_SPEED};
use crate::player::PLAYER_SIZE;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub fn move_enemy(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        transform.translation += enemy.direction * ENEMY_SPEED * time.delta_seconds();
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

// Spawn enemy at random position with random direction over ENEMY_SPAWN_PERIOD time.
pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_timer: Res<EnemyTimer>,
) {
    if enemy_timer.timer.just_finished() {
        let primary_window = window_query.get_single().unwrap();

        // Generate random position and random direction
        let x_pos: f32 = random::<f32>() * primary_window.width();
        let y_pos: f32 = random::<f32>() * primary_window.height();

        let x_dir: f32 = random::<f32>();
        let y_dir: f32 = random::<f32>();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                texture: asset_server.load("sprites/Default/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec3::new(x_dir, y_dir, 0.0).normalize(),
            },
        ));
    }
}

pub fn enemy_timer_tick(mut enemy_timer: ResMut<EnemyTimer>, time: Res<Time>) {
    enemy_timer.timer.tick(time.delta());
}
