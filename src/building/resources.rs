

use bevy::prelude::*;

use crate::all::buildings::components::Building;

#[derive(Resource, Clone)]
pub struct Selected {
    pub val: SelectType, 
}

impl Default for Selected {
    fn default() -> Self {
        Self {
            val: SelectType::None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SelectType {
    Placing(Building),
    Placed(Vec<Vec2>),
    Upgrading,
    Selling,
    None,
}

impl SelectType {
    pub fn id(&self) -> u32 {
        match self {
            SelectType::None => 0,
            SelectType::Placing(_) => 1,
            SelectType::Placed(_) => 2,
            SelectType::Upgrading => 3,
            SelectType::Selling => 4,
        }
    }
}

impl PartialEq for SelectType {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}



