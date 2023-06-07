mod events;
mod game;
mod systems;
mod ui;

use game::GamePlugin;
use systems::*;
use ui::UIPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(GamePlugin)
        .add_plugin(UIPlugin)
        .add_startup_system(spawn_camera)
        .add_system(close_game)
        .add_system(handle_game_over_event)
        .add_system(transition_to_game_state)
        .add_system(transition_to_menu_state)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    Game,
    GameOver,
}
