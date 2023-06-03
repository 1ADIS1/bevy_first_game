use super::ENEMY_SPAWN_PERIOUD;

use ::bevy::prelude::*;

#[derive(Resource)]
pub struct EnemyTimer {
    pub timer: Timer,
}

impl Default for EnemyTimer {
    fn default() -> Self {
        EnemyTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_PERIOUD, TimerMode::Repeating),
        }
    }
}
