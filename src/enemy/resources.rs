use bevy::prelude::{Resource, Timer, TimerMode};

pub const ENEMY_SPAWN_TIME: f32 = 20.0;

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