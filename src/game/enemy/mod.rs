pub mod components;
mod resources;
mod systems;

use super::GameState;
use crate::AppState;
use resources::*;
use systems::*;

use bevy::prelude::*;

pub const ENEMIES_NUM: usize = 4;
pub const ENEMY_SPAWN_PERIOUD: f32 = 5.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 400.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyTimer>()
            // Upon entering the game state, spawn enemies
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            // Upon leaving the game state, despawn enemies
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)))
            .add_systems(
                (
                    move_enemy,
                    update_enemy_direction.after(move_enemy),
                    limit_enemy_movement.after(update_enemy_direction),
                    enemy_timer_tick,
                    spawn_enemies_over_time,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(GameState::Running)),
            );
    }
}
