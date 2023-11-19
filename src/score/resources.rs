use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        return Score { value: 0 };
    }
}


#[derive(Resource)]
#[derive(Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> Self {
        return HighScores { scores: Vec::new() };
    }
}