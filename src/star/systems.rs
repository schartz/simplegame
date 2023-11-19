use bevy::prelude::{AssetServer, Commands, default, Query, Res, ResMut, SpriteBundle, Time, Transform, Window, With};
use bevy::window::PrimaryWindow;
use rand::random;

use crate::star::{STARS_COUNT, STAR_SIZE};
use crate::star::components::Star;
use crate::star::resources::StarSpawnTimer;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let half_star_size = STAR_SIZE / 2.0;
    for _ in 0..STARS_COUNT {
        let mut random_x = random::<f32>() * window.width();
        let mut random_y = random::<f32>() * window.height();

        if random_x < half_star_size {
            random_x = half_star_size
        }
        if random_x > window.width() - half_star_size {
            random_x = window.width() - half_star_size
        }

        if random_y < half_star_size {
            random_y = half_star_size
        }
        if random_y > window.height() - half_star_size {
            random_y = window.height() - half_star_size
        }

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn tick_star_spawn_timer(
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    time: Res<Time>
){
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>
){
    if star_spawn_timer.timer.finished(){
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle{
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {}
        ));
    }
}