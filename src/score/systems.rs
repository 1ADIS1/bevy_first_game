use super::resources::*;
use bevy::prelude::*;

use crate::events::*;

// Prints the score whenever it has changed.
pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Current score: {}", score.value);
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.iter() {
        high_scores
            .scores
            .push(("Player".to_string(), event.score_value));
    }
}

// If the high scores got changed, then print them.
pub fn print_high_scores(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High scores: {:?}", high_scores);
    }
}
