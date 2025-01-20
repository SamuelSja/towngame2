
pub mod building;
pub mod game;
pub mod all;
pub mod menu;


use all::AllPlug;
use bevy::prelude::*;
use building::BuildingPlug;
use game::GamePlug;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(AllPlug) 
    .add_plugins(BuildingPlug)   
    .add_plugins(GamePlug)
    .run();
}
