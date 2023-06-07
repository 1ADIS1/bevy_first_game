mod enemy;
mod player;
mod score;
mod star;
mod systems;

use crate::{events::GameOver, AppState};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_event::<GameOver>()
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            .add_system(toggle_game.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Default, Hash, Eq, PartialEq, Clone)]
pub enum GameState {
    Paused,
    #[default]
    Running,
}
