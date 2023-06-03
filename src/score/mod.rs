pub mod resources;
mod systems;

use resources::*;
use systems::*;

use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_system(update_score)
            .add_system(update_high_scores)
            .add_system(print_high_scores);
    }
}
