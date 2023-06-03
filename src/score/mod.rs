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
            .add_systems((update_score, update_high_scores, print_high_scores));
    }
}
