mod components;
mod systems;

use systems::*;

use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // Player's sprite size.

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((spawn_player,)).add_systems((
            move_player,
            limit_player_movement,
            check_enemy_collision,
            check_star_collision,
        ));
    }
}
