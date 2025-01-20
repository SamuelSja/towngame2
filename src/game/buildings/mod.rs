

pub mod components;
pub mod systems;


use bevy::prelude::*;
use systems::*;

use crate::all::{buildings::components::{BaseBuilding, Ranged}, helper::{enter_either, exit_either}, states::AppState};

use super::{incr_round, next_round};

pub struct BuildingPlug;
impl Plugin for BuildingPlug {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (
            destroy_building,
            building_range_attack,
            town_heal.after(next_round),
            town_rebuild.after(incr_round),
        ).run_if(in_state(AppState::Game)))
        .add_systems(Update, (
            incr_bolt,
        ));
        exit_either(app, (AppState::Game, AppState::GameOver), (
            reset_buildings,
            deconstruct_game,
        ));
        enter_either(app, (AppState::Game, AppState::GameOver), (
            construct_game::<Ranged>,
            construct_game::<BaseBuilding>,
            spawn_health_bars,
        ));
    }
}





