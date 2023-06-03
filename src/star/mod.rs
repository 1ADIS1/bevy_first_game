pub mod components;
mod resources;
mod systems;

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
            .add_startup_system(spawn_star)
            .add_system(spawn_stars_over_time)
            .add_system(star_timer_tick);
    }
}
