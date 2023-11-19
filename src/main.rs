pub mod components;
mod systems;
pub mod resources;
pub mod events;

use resources::*;
use systems::*;
use events::*;


use bevy::prelude::*;
use bevy::window::PresentMode;




fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Simple Game".into(),
                resolution: (1000., 600.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_enemies)
        .add_startup_system(spawn_stars)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemy_movement)
        .add_system(update_enemy_direction)
        .add_system(confine_enemy_movement)
        .add_system(enemy_hit_player)
        .add_system(player_hit_star)
        .add_system(update_score)
        .add_system(tick_star_spawn_timer)
        .add_system(spawn_stars_over_time)
        .add_system(spawn_enemies_over_time)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_system(update_high_scores)
        .add_system(high_scores_updated)
        .run();
}





