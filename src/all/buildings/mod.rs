


pub mod components;
pub mod systems;
pub mod resources;


use bevy::prelude::*;
use systems::update_health_bar;



pub struct BuildingsPlug;

impl Plugin for BuildingsPlug {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, update_health_bar)
        ;
    } 
}




