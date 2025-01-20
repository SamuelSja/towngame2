


use bevy::prelude::*;


#[derive(States, Hash, Eq, PartialEq, Debug, Clone)]
pub enum AppState {
    Menu,
    Build,
    Game,
    GameOver,
}

impl Default for AppState {
    fn default() -> Self {
        Self::Menu
    }
}



