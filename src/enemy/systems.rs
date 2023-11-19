use bevy::prelude::{AssetServer, Audio, Commands, default, Query, Res, ResMut, SpriteBundle, Time, Transform, Vec2, Vec3, Window, With};
use bevy::window::PrimaryWindow;
use rand::random;

use crate::enemy::components::Enemy;
use crate::enemy::resources::EnemySpawnTimer;
// use super::components::Enemy;

use super::{ENEMY_COUNT, ENEMY_SIZE, ENEMY_SPEED};


pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..ENEMY_COUNT {
        let random_x: f32 = random::<f32>() * window.width();
        let random_y: f32 = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large_alt.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min: f32 = 0.0 + half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }

        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        // Play sound effect
        if direction_changed {
            let sound_effect1 = asset_server.load("audio/impactSoft_medium_001.ogg");
            let sound_effect2 = asset_server.load("audio/impactSoft_medium_000.ogg");

            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect1
            } else {
                sound_effect2
            };

            audio.play(sound_effect);
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min: f32 = 0.0 + half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for mut enemy_transform in enemy_query.iter_mut() {
        let mut translation = enemy_transform.translation;

        // Bound the player X position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound the player Y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        enemy_transform.translation = translation;
    }
}

pub fn tick_enemy_spawn_timer(
    mut star_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>
){
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(mut commands: Commands,
                               window_query: Query<&Window, With<PrimaryWindow>>,
                               asset_server: Res<AssetServer>,
                               enemy_spawn_timer: Res<EnemySpawnTimer>
){
    if enemy_spawn_timer.timer.finished(){
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle{
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large_alt.png"),
                ..default()
            },
            Enemy { direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(), }
        ));
    }
}