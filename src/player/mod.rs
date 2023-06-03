mod components;
mod systems;

use systems::*;

use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // Player's sprite size.

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(limit_player_movement.after(move_player))
            .add_system(check_enemy_collision)
            .add_system(check_star_collision);
    }
}
