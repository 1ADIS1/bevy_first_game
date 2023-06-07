mod components;
mod systems;

use super::GameState;
use crate::AppState;

use systems::*;

use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // Player's sprite size.

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Upon entering the game state, spawn player
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            // Upon exitting the game state, despawn player
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)))
            .add_systems(
                (
                    move_player,
                    limit_player_movement.after(move_player),
                    check_enemy_collision,
                    check_star_collision,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(GameState::Running)),
            );
    }
}
