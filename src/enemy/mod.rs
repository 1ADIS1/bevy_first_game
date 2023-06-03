pub mod components;
mod resources;
mod systems;

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
            .add_startup_systems((spawn_enemies,))
            .add_systems((
                move_enemy,
                update_enemy_direction,
                limit_enemy_movement,
                enemy_timer_tick,
                spawn_enemies_over_time,
            ));
    }
}
