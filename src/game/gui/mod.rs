


pub mod top_bar;

use bevy::prelude::*;
use top_bar::*;

use crate::all::states::AppState;





pub struct GUIPlug;
impl Plugin for GUIPlug {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Game), insert_round_label)
        .add_systems(OnExit(AppState::Game), remove_round_label)
        .add_systems(Update, (
            update_round_label
        ).run_if(in_state(AppState::Game)))
        ;
    }
}




