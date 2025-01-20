use std::sync::Arc;

use bevy::{color::palettes::css::BLUE, prelude::*};

use super::build_comps::BuildingComponent;






#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Building {
    None,
    Archer,
    Wall,
    Town,
}

impl Building {
    pub fn spawn(&self, entity: &mut EntityCommands, assets: &Res<AssetServer>) {
        for component in self.components() {
            component.insert(entity, assets);
        }
    }

    pub fn spawn_new<'a>(&self, coms: &'a mut Commands, assets: &Res<AssetServer>) -> EntityCommands<'a> {
        let mut entity: EntityCommands<'a> = coms.spawn_empty();
        self.spawn(&mut entity, assets);
        entity
    }

    pub fn texture(&self, assets: &Res<AssetServer>) -> Handle<Image> {
        assets.load(
            match self {
                Self::None => { "todo.png" },
                Self::Archer => { "images/buildings/archer1.png" },
                Self::Wall => { "images/buildings/wall1.png" },
                Self::Town => { "images/buildings/town1.png" },
            }
        )
    }

    pub fn texture_destroyed(&self, assets: &Res<AssetServer>) -> Handle<Image> {
        assets.load(
            match self {
                Self::None => { "todo.png" },
                Self::Archer => { "images/buildings/destroyed1.png" },
                Self::Wall => { "images/buildings/destroyed1.png" },
                Self::Town => { "images/buildings/destroyed1.png" },
            }
        )
    }

    pub fn name(&self) -> String {
        match self {
            Self::None => { "None" },
            Self::Archer => { "Archer" },
            Self::Wall => { "Wall" },
            Self::Town => { "Town" },
        }.to_string()
    }

    pub fn health(&self) -> f32 {
        match self {
            Self::None => { 0.0 },
            Self::Archer => { 50.0 },
            Self::Wall => { 100.0 },
            Self::Town => { 50.0 },
        }
    }

    pub fn labels(&self, level: u32) -> Vec<(String, f32)> {
        let components = self.components();
        self.components().into_iter().map(|component| {
            component.get_data(level)
        }).flatten().collect()
    }

    pub fn components(&self) -> Vec<BuildingComponent> {
        let mut components = match self {
            Self::None => {vec![]},
            Self::Archer => {vec![
                BuildingComponent::Base { health: Arc::new(|l| { 50.0 * (1.5 as f32).powf(l as f32) }), building: self.clone(), level: 1 },
                BuildingComponent::Ranged { range: Arc::new(|l| { 250.0 * (1.5 as f32).powf(l as f32) }), damage: Arc::new(|l| { 20.0 * (1.5 as f32).powf(l as f32 - 1.0) }), reload_time: Arc::new(|l| {
                    1.0 * ((2.0 / 3.0) as f32).powf(l as f32 - 1.0)
                }), bolt_color: Arc::new(|l| { BLUE }) },
                BuildingComponent::Solid {},
                BuildingComponent::Buyable { cost: Arc::new(|l| { (20.0 * l as f32) as u32 })},
            ]},
            Self::Wall => {vec![
                BuildingComponent::Base { health: Arc::new(|l| { 100.0 * (1.5 as f32).powf(l as f32) }), building: self.clone(), level: 1 },
                BuildingComponent::Solid {},
                BuildingComponent::Buyable { cost: Arc::new(|l| { (5.0 * l as f32) as u32 })},
            ]},
            Self::Town => {vec![
                BuildingComponent::Base { health: Arc::new(|l| { 50.0 * (1.5 as f32).powf(l as f32) }), building: self.clone(), level: 1 },
                BuildingComponent::Town { power: Arc::new(|l| { 0.1 * (1.5 as f32).powf(l as f32) }), range: Arc::new(|l| { 8.0 * (1.5 as f32).powf(l as f32) }) },
                BuildingComponent::Solid {},
                BuildingComponent::Buyable { cost: Arc::new(|l| { (20.0 * l as f32) as u32 })},
            ]}

        };
        components
    }

    pub fn cost(&self, level: u32) -> Option<u32> {
        let comps = self.components();

        for comp in comps.iter() {
            if let BuildingComponent::Buyable { cost } = comp {
                return Some(cost(level));
            }
        } 

        None
    }
}