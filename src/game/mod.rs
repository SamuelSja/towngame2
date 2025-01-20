
pub mod asset_consts;
pub mod buildings;
pub mod enemies;
pub mod systems;
pub mod resources;
pub mod events;
pub mod gui;

use bevy::prelude::*;
use buildings::BuildingPlug;
use enemies::{systems::spawn_enemies, EnemyPlug};
use events::NextRound;
use gui::GUIPlug;
use resources::Round;
use systems::*;

use crate::all::states::AppState;
// use systems::buildings::{building_range_attack, destroy_building};

pub struct GamePlug;
impl Plugin for GamePlug {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(EnemyPlug)
        .add_plugins(BuildingPlug)
        .add_plugins(GUIPlug)
        .add_event::<NextRound>()
        .init_resource::<Round>()
        .add_systems(Update, (
            next_round.before(spawn_enemies),
            incr_round,
        ).chain().run_if(in_state(AppState::Game)))
        .add_systems(Update, (
            game_over_when_no_town,
        ).run_if(in_state(AppState::Game)))
        .add_systems(OnExit(AppState::Game), reset_round)
        ;
    }
}

