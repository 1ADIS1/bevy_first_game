use super::components::Player;
use super::{PLAYER_SIZE, PLAYER_SPEED};
use crate::events::GameOver;
use crate::game::enemy::components::*;
use crate::game::enemy::ENEMY_SIZE;
use crate::game::score::resources::*;
use crate::game::star::components::*;
use crate::game::star::STAR_SIZE;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// Update player transform every frame.
pub fn move_player(
    time: Res<Time>,
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

    player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
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

// Checks if the player and enemy collide with each other.
pub fn check_enemy_collision(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut game_over_event_writer: EventWriter<GameOver>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>,
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
            // Send GameOver event.
            game_over_event_writer.send(GameOver {
                score_value: score.value,
            });

            // Play the sound.
            let sound_effect = asset_server.load("audio/scifi/explosionCrunch_000.ogg");
            audio.play(sound_effect);

            // Despawn player.
            commands.entity(player_entity).despawn();
        }
    }
}

// Checks if the player and star collide with each other.
pub fn check_star_collision(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(&Transform, Entity), With<Star>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_radius = PLAYER_SIZE / 2.0;

        for (star_transform, star_entity) in star_query.iter() {
            let star_radius = STAR_SIZE / 2.0;

            if player_transform
                .translation
                .distance(star_transform.translation)
                < player_radius + star_radius
            {
                // Play the star sound, increment the score, and despawn the star.
                let star_sound = asset_server.load("audio/interface/confirmation_001.ogg");
                audio.play(star_sound);

                score.value += 1;

                commands.entity(star_entity).despawn();
            }
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

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    let player_entity = match player_query.get_single() {
        Ok(player_entity) => player_entity,
        Err(_) => return,
    };

    commands.entity(player_entity).despawn();
}
