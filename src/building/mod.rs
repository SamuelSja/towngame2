pub mod gui;
pub mod systems;
pub mod resources;
pub mod components;

use bevy::prelude::*;
use gui::GUIPlug;
use resources::Selected;
use systems::*;

use crate::all::states::AppState;

pub struct BuildingPlug;
impl Plugin for BuildingPlug {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(GUIPlug)
        .init_resource::<Selected>()
        // .add_systems(Startup, test_spawn_building)
        .add_systems(Update, (
            clicked,
            // select_on_grid,
            deselect_on_esc,
        ).run_if(in_state(AppState::Build)))
        .add_systems(OnExit(AppState::Build), deselect)
        ;
    }
}




