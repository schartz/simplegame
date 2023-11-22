use bevy::app::App;
use bevy::prelude::{IntoSystemConfig, IntoSystemSetConfig, Plugin, SystemSet};
use crate::player::systems::{confine_player_movement, enemy_hit_player, player_hit_star,
                             player_movement, spawn_player};


pub mod components;
mod systems;

pub struct PlayerPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

/*#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet{
    Movement,
    Confinement
}

app
    .configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
    .add_system(player_movement.in_set(PlayerSystemSet::Movement))
    .add_system(confine_player_movement.in_set(PlayerSystemSet::Confinement))*/

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
            .configure_set(MovementSystemSet.before(ConfinementSystemSet))
            .add_startup_system(spawn_player)
            .add_system(player_movement.in_set(MovementSystemSet))
            .add_system(confine_player_movement.in_set(ConfinementSystemSet))
            .add_system(enemy_hit_player)
            .add_system(player_hit_star);
    }
}