
pub mod components;
pub mod layout;
pub mod styles;
pub mod interactions;

use bevy::prelude::*;
use interactions::update_gold;
use layout::spawn_gui;

pub struct GUIPlug;

impl Plugin for GUIPlug {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_gui)
        .add_systems(Update, (
            update_gold,
        ))
        ;
    }
}



