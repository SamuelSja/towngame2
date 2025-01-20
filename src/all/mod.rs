
pub mod asset_consts;
pub mod gui;
pub mod buildings;
pub mod systems;
pub mod helper;
pub mod states;
pub mod resources;

use bevy::prelude::*;
use buildings::BuildingsPlug;
use gui::GUIPlug;
use resources::{Gold, PrevState};
use states::AppState;
use systems::*;

pub const TILE_SIZE: Vec2 = Vec2::new(50.0, 50.0);
pub const CAMERA_MOVE_SPEED: f32 = 10.0;
pub const BUILDABLE_SIZE: (usize, usize) = (20, 20);

pub const GROUND_IMAGE_1: &str = "images/map/grass_green.png";
pub const GROUND_IMAGE_2: &str = "images/map/grass_orange.png";
pub const BLANK_IMAGE: &str = "images/blank.png";


pub struct AllPlug;

impl Plugin for AllPlug {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(BuildingsPlug)
        .add_plugins(GUIPlug)
        .init_state::<AppState>()
        .init_resource::<Gold>()
        .init_resource::<PrevState>()
        .add_systems(Startup, (
            spawn_camera,
            spawn_ground,
        ))
        .add_systems(Update, (
            move_camera,
            update_prev_state,
        ))
        .add_systems(Update, test_switch_states)
        ;
    } 
}


