use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub value: usize,
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, usize)>,
}

impl Default for HighScores {
    fn default() -> Self {
        HighScores { scores: Vec::new() }
    }
}
