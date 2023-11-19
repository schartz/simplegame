use bevy::prelude::{Resource, Timer, TimerMode};


pub const STAR_SPAWN_TIME: f32 = 1.0;
pub const ENEMY_SPAWN_TIME: f32 = 20.0;

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


#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}
impl Default for StarSpawnTimer {
    fn default() -> Self {
        return StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        };
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        return EnemySpawnTimer{
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}