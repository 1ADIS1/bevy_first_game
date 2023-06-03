use super::STAR_SPAWN_PERIOD;

use bevy::prelude::*;

#[derive(Resource)]
pub struct StarTimer {
    pub timer: Timer,
}

impl Default for StarTimer {
    fn default() -> Self {
        StarTimer {
            timer: Timer::from_seconds(STAR_SPAWN_PERIOD, TimerMode::Repeating),
        }
    }
}
