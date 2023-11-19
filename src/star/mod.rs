use bevy::app::{App, Plugin};
use crate::star::resources::StarSpawnTimer;
use crate::star::systems::{spawn_stars, spawn_stars_over_time, tick_star_spawn_timer};

pub mod components;
mod resources;
mod systems;

pub const STARS_COUNT: usize = 10;
pub const STAR_SIZE: f32 = 30.0;

pub struct StarPlugin;

impl Plugin for StarPlugin{
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_startup_system(spawn_stars)
            .add_system(tick_star_spawn_timer)
            .add_system(spawn_stars_over_time);
    }
}