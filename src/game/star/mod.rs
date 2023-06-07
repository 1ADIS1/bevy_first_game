pub mod components;
mod resources;
mod systems;

use super::GameState;
use crate::AppState;
use resources::*;
use systems::*;

use bevy::prelude::*;

pub const STARS_NUM: usize = 10;
pub const STAR_SPAWN_PERIOD: f32 = 1.0;
pub const STAR_SIZE: f32 = 30.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarTimer>()
            .add_system(spawn_star.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)))
            .add_systems(
                (spawn_stars_over_time, star_timer_tick)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(GameState::Running)),
            );
    }
}
