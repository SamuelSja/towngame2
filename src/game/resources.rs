
use bevy::prelude::*;





#[derive(Resource)]
pub struct Round {
    pub val: u32,
}

impl Default for Round {
    fn default() -> Self {
        Self {
            val: 0,
        }
    }
}
























