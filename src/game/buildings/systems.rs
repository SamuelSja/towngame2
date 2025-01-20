

use std::{i32, usize};

use bevy::prelude::*;

use crate::{all::{buildings::{components::{BaseBuilding, BaseBuildingGame, BuildComponent, HealthBarBack, HealthBarFront, Ranged, RangedGame, Town}, systems::insert_health_bar}, BLANK_IMAGE, TILE_SIZE}, game::{asset_consts::LASER_SOUNDS, enemies::components::{Base, Enemy}, events::NextRound, next_round, resources::Round}};

use super::components::Bolt;


pub fn reset_buildings (
    mut buildings_q: Query<(&Children, &mut Sprite, &mut BaseBuilding, &mut Transform)>,
    mut building_game_q: Query<&mut BaseBuildingGame>,
    assets: Res<AssetServer>,
) {
    for (children, mut sprite, mut base, mut transform) in buildings_q.iter_mut() {
        for &child in children.iter() {
            if let Ok(mut building_game) = building_game_q.get_mut(child) {
                sprite.image = base.building.texture(&assets);
                building_game.death_updated = false;
                transform.translation.z = 0.0;
                building_game.health = (base.max_health)(base.level);
            }
        }
    }
}

pub fn destroy_building (
    mut commands: Commands,
    mut building_q: Query<(&Children, &mut Sprite, &mut BaseBuilding, &mut Transform)>,
    assets: Res<AssetServer>,
    mut base_game_q: Query<&mut BaseBuildingGame>,
) {
    for (children, mut sprite, mut base, mut transform) in building_q.iter_mut() {
        for &child in children {
            if let Ok(mut base_game) = base_game_q.get_mut(child) {
                if ! base_game.alive() && ! base_game.death_updated {
                    base_game.death_updated = true;
                    sprite.image = base.building.texture_destroyed(&assets);
                    transform.translation.z = -0.3;
                }
            }
        }
    }
}

pub fn building_range_attack (
    mut coms: Commands,
    mut building_q: Query<(&Children, &Transform, &BaseBuilding, &mut Ranged)>,
    mut ranged_q: Query<&mut RangedGame>,
    mut base_q: Query<&BaseBuildingGame>, 
    mut enemy_q: Query<(&Transform, &mut Base)>,
    assets: Res<AssetServer>,
    time: Res<Time>,
) {
    'main:
    for (children, building, base_building, mut ranged) in building_q.iter_mut() {
        for &child in children {
            
            if let Ok(base_game) = base_q.get(child) {
                if ! base_game.alive() {
                    continue 'main;
                }
            }

            if let Ok(mut ranged_game) = ranged_q.get_mut(child) {
                if ranged_game.cur_reload > 0.0 {
                    ranged_game.cur_reload -= time.delta_secs();
                    continue 'main;
                }


                let building_translation = building.translation;
                let building_vec2 = Vec2::new(building_translation.x, building_translation.y);

                let mut closest = usize::MAX;
                let mut dist = f32::MAX;

                let mut i = 0;
                for (enemy, mut base) in enemy_q.iter_mut() {
                    let enemy_translation = enemy.translation;
                    let enemy_vec2 = Vec2::new(enemy_translation.x, enemy_translation.y);

                    let possible_dist = building_vec2.distance(enemy_vec2);
                    if possible_dist < dist {
                        closest = i;
                        dist = possible_dist;
                    }

                    i += 1;
                }


                if dist > (ranged.range)(base_building.level) {
                    continue;
                }

                let (enemy, mut base) = enemy_q.iter_mut().nth(closest).unwrap();
                base.health -= (ranged.damage)(base_building.level);
                ranged_game.cur_reload = (ranged.reload_time)(base_building.level);

                // Section: draw bolt
                let mut transform = Transform::from_xyz(0.0, 0.0, 0.0);
                transform.translation = enemy.translation + (building.translation - enemy.translation) / 2.0;
                transform.translation.z = 1.0;
                transform.scale = Vec3::new(dist, 10.0, 0.0);
                transform.rotate_z({
                    let x = enemy.translation.x - building.translation.x;
                    let y = enemy.translation.y - building.translation.y;
                    (y / x).atan()
                });
                coms.spawn((
                    Sprite {
                        image: assets.load(BLANK_IMAGE),
                        color: (ranged.bolt_color)(base_building.level).into(),
                        ..default()
                    },
                    transform,
                    Bolt::default(),
                ));

                // Section: sound effect
                coms.spawn((
                    AudioPlayer::new(assets.load(LASER_SOUNDS[0])),
                    PlaybackSettings::DESPAWN,
                ));
            }
        }
        
    }
}


// Todo: visual for the healing
pub fn town_heal (
    mut coms: Commands,
    town_q: Query<(&Children, &Transform, &Town, &BaseBuilding)>,
    mut building_q: Query<(&Children, &Transform, &mut BaseBuilding), Without<Town>>,
    mut base_game_q: Query<&mut BaseBuildingGame>,
    mut next_round_reader: EventReader<NextRound>,
    assets: Res<AssetServer>,
) {
    for _ in next_round_reader.read() {
        for (children, town_transform, town, base) in town_q.iter() {
            for &child in children {
                if let Ok(town_base_game) = base_game_q.get(child) {
                    if ! town_base_game.alive() {
                        continue;
                    }

                    for (children, building_transform, mut base) in building_q.iter_mut() {
                        for &child in children {
                            if let Ok(mut base_game) = base_game_q.get_mut(child) {
                                if ! base_game.alive() {
                                    continue;
                                }

                                let town_translation = town_transform.translation;
                                let building_translation = building_transform.translation;

                                let town_vec2 = Vec2::new(town_translation.x, town_translation.y);
                                let building_vec2 = Vec2::new(building_translation.x, building_translation.y);

                                let dist = town_vec2.distance(building_vec2);

                                if (dist / TILE_SIZE.x) <= (town.range)(base.level) {
                                    let mut cur_health = base_game.health;
                                    cur_health += (base.max_health)(base.level) * (town.power)(base.level);

                                    if cur_health >= (base.max_health)(base.level) {
                                        base_game.health = (base.max_health)(base.level);
                                    } else {
                                        base_game.health = cur_health;
                                    }
                                }

                            }
                        }
                    }
                }
            }
        }
    }
}

// Todo: visual for the rebuilding
pub fn town_rebuild (
    mut coms: Commands,
    town_q: Query<(&Children, &Transform, &Town, &BaseBuilding)>,
    mut building_q: Query<(&Children, &mut Transform, &mut BaseBuilding, &mut Sprite), Without<Town>>,
    mut base_game_q: Query<&mut BaseBuildingGame>,
    mut next_round_reader: EventReader<NextRound>,
    round: Res<Round>,
    assets: Res<AssetServer>,
) {
    for _ in next_round_reader.read() {
        if round.val % 10 == 0 {
            for (children, town_transform, town, base) in town_q.iter() {
                for &child in children { if let Ok(town_base_game) = base_game_q.get(child) {
                    if ! town_base_game.alive() {
                        continue;
                    }

                    for (children, mut building_transform, mut base, mut sprite) in building_q.iter_mut() {
                        for &child in children { if let Ok(mut base_game) = base_game_q.get_mut(child) {
                            if base_game.alive() {
                                continue;
                            }
                            let town_translation = town_transform.translation;
                            let building_translation = building_transform.translation;

                            let town_vec2 = Vec2::new(town_translation.x, town_translation.y);
                            let building_vec2 = Vec2::new(building_translation.x, building_translation.y);

                            let dist = town_vec2.distance(building_vec2);

                            if (dist / TILE_SIZE.x) <= (town.range)(base.level) {
                                let mut cur_health = base_game.health;
                                cur_health += (base.max_health)(base.level) * (town.power)(base.level);

                                if cur_health >= (base.max_health)(base.level) {
                                    base_game.health = (base.max_health)(base.level);
                                } else {
                                    base_game.health = cur_health;
                                }

                                building_transform.translation.z = 0.0;
                                sprite.image = base.building.texture(&assets);


                                base_game.check_life();
                            }
                        }}
                    }
                }}
            }
        }
    }
}

pub fn incr_bolt (
    mut coms: Commands,
    mut bolt_q: Query<(Entity, &mut Bolt)>,
    time: Res<Time>,
) {
    for (entity, mut bolt) in bolt_q.iter_mut() {
        bolt.time_left -= time.delta_secs();

        if bolt.time_left <= 0.0 {
            coms.get_entity(entity).unwrap().despawn();
        }
    }
}

pub fn construct_game<T: BuildComponent + Component> (
    mut coms: Commands,
    components_q: Query<(Entity, &T, &BaseBuilding)>, 
) {
    for (entity, comp, base) in components_q.iter() {
        coms.get_entity(entity).unwrap().with_children(|parent| {
            comp.spawn_game_component(parent, base.level);
        });
    }
}

/// This will del all descendants of BaseBuildings
pub fn deconstruct_game (
    mut coms: Commands,
    components_q: Query<Entity, With<BaseBuilding>>, 
) {
    for entity in components_q.iter() {
        coms.get_entity(entity).unwrap().despawn_descendants();
    }
}

/// Spawns health bars on all existing buildings
pub fn spawn_health_bars (
    mut coms: Commands,
    building_q: Query<Entity, With<BaseBuilding>>,
    assets: Res<AssetServer>,
) {
    println!("spawn health bar");
    for entity in building_q.into_iter() {
        insert_health_bar(&mut coms.get_entity(entity).unwrap(), &assets);
    }
}

// /// Removes all health bars
// pub fn despawn_health_bars (
//     mut coms: Commands,
//     back_q: Query<Entity, With<HealthBarBack>>,
//     front_q: Query<Entity, With<HealthBarFront>>,
// ) {


//     for entity in back_q.iter() {
//         coms.get_entity(entity).unwrap().despawn_recursive();
//     }

//     for entity in front_q.iter() {
//         coms.get_entity(entity).unwrap().despawn_recursive();
//     }
// }