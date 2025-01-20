


use bevy::prelude::*;


#[derive(Component)]
pub struct Bolt {
    pub time_left: f32,
}

impl Default for Bolt {
    fn default() -> Self {
        Self {
            time_left: 0.5,
        }
    }
}









