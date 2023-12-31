use bevy::app::App;
use bevy::prelude::Plugin;
use crate::enemy::resources::EnemySpawnTimer;
use crate::enemy::systems::{confine_enemy_movement, enemy_movement, spawn_enemies, update_enemy_direction, spawn_enemies_over_time, tick_enemy_spawn_timer};

pub mod components;
pub mod resources;
mod systems;

pub const ENEMY_COUNT: usize = 2;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;


pub struct EnemyPlugin;
impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_startup_system(spawn_enemies)
            .add_system(enemy_movement)
            .add_system(update_enemy_direction)
            .add_system(confine_enemy_movement)
            .add_system(spawn_enemies_over_time)
            .add_system(tick_enemy_spawn_timer);
    }
}

