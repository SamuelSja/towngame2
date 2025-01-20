// pub mod buildings;


use bevy::prelude::*;

use crate::all::{buildings::{self, components::{BaseBuildingGame, Building, Town}}, states::AppState};

use super::{enemies, events::NextRound, resources::Round};

type EBase = enemies::components::Base;
type BBase = buildings::components::BaseBuilding;

pub fn next_round (
    enemy_q: Query<(), With<EBase>>,
    mut next_round_writer: EventWriter<NextRound>
) {
    if enemy_q.is_empty() {
        next_round_writer.send(NextRound {});
    }
}

pub fn incr_round (
    mut next_round_reader: EventReader<NextRound>,
    mut round: ResMut<Round>,
) {
    for _ in next_round_reader.read() {
        round.val += 1;
    }
}

pub fn reset_round (
    mut round: ResMut<Round>,
) {
    *round = Round::default();
}

// Todo: This method
pub fn game_over_when_no_town (
    towns_q: Query<&Children, With<Town>>,
    base_games_q: Query<&BaseBuildingGame>,
    mut app_state: ResMut<NextState<AppState>>,
) {

    let mut town = false;

    for children in towns_q.iter() {
        for child in children {
            if let Ok(base_game) = base_games_q.get(*child) {
                town |= base_game.alive();
            }
        }
    }

    if ! town {
        app_state.set(AppState::GameOver);
    }
}
