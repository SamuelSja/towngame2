

use bevy::prelude::*;


use std::sync::Arc;

use super::build_types::Building;



#[derive(Component)]
pub struct BaseBuilding {
    // pub health: f32,
    pub max_health: Arc<dyn Fn(u32) -> f32 + Send + Sync>,
    pub building: Building,
    pub level: u32,
}

impl BaseBuilding {
    pub fn level_up(&mut self, levels: u32) {
        self.level += levels;
    }
}

#[derive(Component)]
pub struct BaseBuildingGame {
    pub health: f32,
    pub death_updated: bool,
}

impl BaseBuildingGame {
    pub fn alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn check_life(&mut self) {
        if self.alive() {
            self.death_updated = false;
        }
    }
}

impl BuildComponent for BaseBuilding {
    fn spawn_game_component(&self, parent: &mut ChildBuilder, level: u32) {
        parent.spawn((
            BaseBuildingGame {
                health: (self.max_health)(level),
                death_updated: false,
            },
        ));
    }
}

#[derive(Component)]
pub struct Ranged {
    pub range: Arc<dyn Fn(u32) -> f32 + Send + Sync>,
    pub damage: Arc<dyn Fn(u32) -> f32 + Send + Sync>,
    pub reload_time: Arc<dyn Fn(u32) -> f32 + Send + Sync>,
    // pub cur_reload: f32,
    pub bolt_color: Arc<dyn Fn(u32) -> Srgba + Send + Sync>,
}

#[derive(Component)]
pub struct RangedGame {
    pub cur_reload: f32,
}

impl BuildComponent for Ranged {
    fn spawn_game_component(&self, parent: &mut ChildBuilder, level: u32) {
        parent.spawn((
            RangedGame {
                cur_reload: (self.reload_time)(level),
            },
        ));
    }
}

#[derive(Component)]
pub struct Town {
    pub power: Arc<dyn Fn(u32) -> f32 + Send + Sync>,
    pub range: Arc<dyn Fn(u32) -> f32 + Send + Sync>,
}

impl BuildComponent for Town {}

#[derive(Component)]
pub struct Solid;

impl BuildComponent for Solid {}


#[derive(Component)]
pub struct Map;

impl BuildComponent for Map {}


#[derive(Component)]
pub struct HealthBarFront;

#[derive(Component)]
pub struct HealthBarBack;








pub trait BuildComponent {
    fn spawn_game_component(&self, _parent: &mut ChildBuilder, _level: u32) {}
}






