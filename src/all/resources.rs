

use bevy::prelude::*;

use super::states::AppState;


#[derive(Resource)]
pub struct Gold {
    pub val: u32,
}

impl Default for Gold {
    fn default() -> Self {
        Self {
            val: 10000,
        } 
    }
}

#[derive(Resource, Clone)]
pub struct PrevState(pub Option<AppState>, pub AppState);
impl Default for PrevState {
    fn default() -> Self {
        Self(None, AppState::Menu)
    }
}