
use bevy::prelude::*;

use crate::{all::{buildings::{components::{BaseBuilding, Building, BuildingComponent}, systems::{at, is_free}}, resources::Gold, TILE_SIZE}, building::resources::{SelectType, Selected}};


pub fn spawn_building(commands: &mut Commands, assets: &Res<AssetServer>, building: Building, pos: Vec2) {
    let entity = building.spawn_new(commands, assets)
    .insert((
        Sprite {
            image: building.texture(assets),
            ..default()
        },
        Transform {
            translation: Vec3::new(pos.x * TILE_SIZE.x, pos.y * TILE_SIZE.y, 0.0),
            ..default()
        },
    ));
}

pub fn mouse_tile(window: &Window, camera: &Transform) -> Option<Vec2> {
    let mut mouse_pos = window.cursor_position()?;
    mouse_pos.y *= -1.0;
    mouse_pos.y += window.height();

    let translation = camera.translation;
    mouse_pos += Vec2::new(translation.x, translation.y) - window.size() / 2.0;

    Some((mouse_pos / TILE_SIZE).round())
}


pub fn place_single_building(coms: &mut Commands, assets: &Res<AssetServer>, building: &Building, tile: Vec2, gold: &mut ResMut<Gold>) {
    let comps = building.components();
    let mut cost: Option<u32> = None;
    for comp in comps {
        if let BuildingComponent::Buyable { cost: cost_fn } = comp {
            cost = Some(cost_fn(1));
        }
    }

    if let Some(cost) = cost {
        if gold.val >= cost {
            gold.val -= cost;

            spawn_building(coms, &assets, building.clone(), tile);
        }
    } 
}


pub fn sell (
    coms: &mut Commands,
    buildings_q: &Query<(&Transform, &BaseBuilding, Entity)>,
    gold: &mut ResMut<Gold>,
    tile: Vec2,
) {
        for (transform, base, entity) in buildings_q.into_iter() {
            if at(transform, &tile) {
                if let Some(sell_amount) = base.building.cost(base.level) {
                    gold.val += sell_amount;
                    coms.get_entity(entity).unwrap().despawn_recursive();
                }
            }
        }
        // let building = entity_at(&buildings_q, &tile).expect("should only have tile if building on tile");
}