
pub mod helper;

use bevy::{math::VectorSpace, prelude::*};
use helper::{spawn_enemy, spawn_enemy_around};

use crate::{all::{asset_consts::ENEMY_ATTACK_SOUNDS, buildings::{self, components::{BaseBuilding, BaseBuildingGame, Town}}, helper::{did_collide, restrict_transform_movement}, resources::Gold, BUILDABLE_SIZE, TILE_SIZE}, game::{events::NextRound, resources::Round}};
use super::components::{self, Attack, Base, Drops, Enemy};

type EMove = components::Move;
type ESolid = components::Solid;
type BSolid = buildings::components::Solid;


pub fn move_enemy (
    mut enemy_q: Query<(&mut Transform, &EMove)>,
    town_q: Query<(&Children, &Transform, &BaseBuilding), (With<Town>, Without<EMove>)>,
    base_game_q: Query<&BaseBuildingGame>,
) {

    for (mut enemy_transform, enemy_move) in enemy_q.iter_mut() {
        let enemy_translation = enemy_transform.translation;
        let enemy_pos = Vec2::new(enemy_translation.x, enemy_translation.y);

        let mut closest = Vec2::new(f32::MAX, f32::MAX);
        if town_q.is_empty() {
            closest = enemy_pos;
        }

        for (children, town, base) in town_q.iter() {
            for &child in children { if let Ok(base_game) = base_game_q.get(child) {
                if ! base_game.alive() {
                    continue;
                }

                let translation = town.translation;
                let pos = Vec2::new(translation.x, translation.y);

                if enemy_pos.distance(pos) < enemy_pos.distance(closest) {
                    closest = pos;
                }
            }}
        }


        let pos_vec = closest - enemy_pos;

        let move_amount = pos_vec.normalize_or_zero() * enemy_move.speed;
        enemy_transform.translation += Vec3::new(move_amount.x, move_amount.y, 0.0);
    }
}

pub fn restrict_enemy_movement (
    mut enemy_q: Query<&mut Transform, With<ESolid>>,
    buildings_q: Query<(&Children, &Transform, &BaseBuilding), (With<BSolid>, Without<ESolid>)>,
    base_game_q: Query<&BaseBuildingGame>,
) {
    for mut enemy in enemy_q.iter_mut() {
        for (children, building, base) in buildings_q.iter() {
            for &child in children { if let Ok(base_game) = base_game_q.get(child) {
                if ! base_game.alive() {
                    continue;
                }
                restrict_transform_movement(&mut enemy, building);
            }}
        }
    } 
}

pub fn hurt_building (
    mut coms: Commands,
    mut enemy_q: Query<(&Transform, &mut Attack)>,
    mut buildings_q: Query<(&Children, &mut BaseBuilding, &Transform)>,
    mut base_game_q: Query<&mut BaseBuildingGame>,
    time: Res<Time>,
    assets: Res<AssetServer>,
) {



    for (enemy_transform, mut attack) in enemy_q.iter_mut() {


        if attack.cur_reload <= 0.0 {

            'build_loop:
            for (children, mut base, building_transform) in buildings_q.iter_mut() {
                for &child in children.iter() { if let Ok(mut base_game) = base_game_q.get_mut(child) {
                    if ! base_game.alive() {
                        continue;
                    }
                    if did_collide(enemy_transform.translation, TILE_SIZE, building_transform.translation, TILE_SIZE) {
                        base_game.health -= attack.damage;
                        attack.cur_reload += attack.reload;

                        coms.spawn((
                            AudioPlayer::new(assets.load(ENEMY_ATTACK_SOUNDS[0])),
                            PlaybackSettings::DESPAWN,
                        ));

                        break 'build_loop;



                    }
                }}
            }
        } else {
            attack.cur_reload -= time.delta_secs();
        }


    } 
}

pub fn spawn_enemies (
    mut coms: Commands,
    mut next_round_reader: EventReader<NextRound>,
    assets: Res<AssetServer>,
    round: Res<Round>,
) {
    for _ in next_round_reader.read() {
        for _ in 0..10 {
            spawn_enemy_around(&mut coms, &assets, Enemy::Normal, BUILDABLE_SIZE, round.val);
        }
    }
}

pub fn despawn_enemies (
    mut coms: Commands,
    enemy_q: Query<Entity, With<Base>>,
) {
    for entity in enemy_q.iter() {
        coms.get_entity(entity).unwrap().despawn();
    }
}

pub fn kill_enemies (
    mut coms: Commands,
    enemy_q: Query<(Entity, &Base, &Drops)>,
    mut gold: ResMut<Gold>,
) {
    for (entity, base, drops) in enemy_q.iter() {
        if base.health <= 0.0 {
            coms.get_entity(entity).unwrap().despawn();

            // Todo: gold should scale with round
            gold.val += drops.gold;
        }
    }
}




