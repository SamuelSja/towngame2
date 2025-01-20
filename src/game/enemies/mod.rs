


pub mod components;
pub mod systems;



use bevy::prelude::*;
use systems::*;

use crate::all::{helper::exit_either, states::AppState};

use super::next_round;




pub struct EnemyPlug;
impl Plugin for EnemyPlug {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, spawn_enemies.after(despawn_enemies));
        // .add_systems(OnExit(AppState::Game), despawn_enemies.run_if(
        //     in_state(AppState::GameOver).map(|val| ! val)
        // ))
        // .add_systems(OnExit(AppState::GameOver), despawn_enemies.run_if(
        //     in_state(AppState::Game).map(|val| ! val)
        // ))
        exit_either(app, (AppState::Game, AppState::GameOver), despawn_enemies)

        .add_systems(Update, (
            move_enemy,
            hurt_building,
            restrict_enemy_movement,
            kill_enemies,
        ).chain().run_if(in_state(AppState::Game)))
        ;
    }
}




