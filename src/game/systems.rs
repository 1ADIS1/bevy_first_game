use super::GameState;

use bevy::prelude::*;

pub fn toggle_game(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    if game_state.0 == GameState::Running {
        commands.insert_resource(NextState(Some(GameState::Paused)));
        println!("Game paused!");
    }
    if game_state.0 == GameState::Paused {
        commands.insert_resource(NextState(Some(GameState::Running)));
        println!("Game running!");
    }
}
