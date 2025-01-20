

use std::sync::Arc;

use bevy::prelude::*;


use super::{bevy_comps::*, build_types::Building};


pub enum BuildingComponent {
    Base { health: Arc<dyn Fn(u32) -> f32 + Send + Sync>, building: Building, level: u32 },
    Ranged { range: Arc<dyn Fn(u32) -> f32 + Send + Sync>, damage: Arc<dyn Fn(u32) -> f32 + Send + Sync>, reload_time: Arc<dyn Fn(u32) -> f32 + Send + Sync>, bolt_color: Arc<dyn Fn(u32) -> Srgba + Send + Sync> },
    Town { power: Arc<dyn Fn(u32) -> f32 + Send + Sync>, range: Arc<dyn Fn(u32) -> f32 + Send + Sync> },
    Solid,
    Buyable { cost: Arc<dyn Fn(u32) -> u32 + Send + Sync> },
}

impl BuildingComponent {
    pub fn get_data(&self, level: u32) -> Vec<(String, f32)> {
        match self {
            Self::Base { health, building: _, level: _} => {
                vec![
                    (
                        String::from("Health"),
                        health(level),
                    ),
                    (
                        String::from("Level"),
                        level as f32,
                    )
                ]
            },
            Self::Ranged { range, damage, reload_time, bolt_color: _ } => {
                vec![
                    (
                        String::from("Range"),
                        range(level),
                    ),
                    (
                        String::from("Damage"),
                        damage(level),
                    ),
                    (
                        String::from("Reload Time"),
                        reload_time(level),
                    )
                ]
            },
            Self::Town { power, range } => {
                vec![
                    (
                        String::from("Power"),
                        power(level),
                    ),
                    (
                        String::from("Range"),
                        range(level),
                    )
                ]
            },
            Self::Buyable { cost } => {
                vec![
                    (
                        String::from("Cost"),
                        cost(level) as f32,
                    )
                ]
            }
            _ => Vec::new(),
        }
    }

    pub fn insert(&self, entity: &mut EntityCommands, assets: &Res<AssetServer>) {
        match self {
            Self::Base { health, building, level } => {
                entity.insert(
                    BaseBuilding {
                        // health: health(1),
                        max_health: health.clone(),
                        building: building.clone(),
                        // death_updated: false,
                        level: level.clone(), 
                    }
                );
                // insert_health_bar(entity, assets);
            },
            Self::Ranged { range, damage, reload_time, bolt_color } => {
                entity.insert(
                    Ranged {
                        range: range.clone(),
                        damage: damage.clone(),
                        reload_time: reload_time.clone(),
                        // cur_reload: reload_time(1),
                        bolt_color: bolt_color.clone(),
                    } 
                );
            },
            Self::Town { power , range} => {
                entity.insert(
                    Town {
                        power: power.clone(),
                        range: range.clone(),
                    }
                );
            },
            Self::Solid => {
                entity.insert(
                    Solid {},
                );
            },
            Self::Buyable { cost: _ } => {
                // Inserting nothing because buyable will always be the same for each building, so I 
                // will just get the buyable of the building and calculate on that
            },
        };
    }

}

