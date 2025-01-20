

use bevy::prelude::*;
use rand::random;

use crate::{all::TILE_SIZE, game::enemies::components::Enemy};

pub fn spawn_enemy(commands: &mut Commands, assets: &Res<AssetServer>, enemy: Enemy, pos: Vec2, round: u32) {
    let entity = enemy.spawn_new(commands, round)
    .insert((
        Sprite {
            image: enemy.texture(assets),
            ..default()
        },
        Transform {
            translation: Vec3::new(pos.x * TILE_SIZE.x, pos.y * TILE_SIZE.y, 0.0),
            ..default()
        },
    ));
}

pub fn spawn_enemy_around(commands: &mut Commands, assets: &Res<AssetServer>, enemy: Enemy, map_dems: (usize, usize), round: u32) {
    let map_dems = (map_dems.0 + 1, map_dems.1 + 1);
    let perim = 2 * (map_dems.0 + map_dems.1);
    let rand = (random::<f32>() * perim as f32) as usize;

    let (x, y) = if rand < map_dems.0 * 2 {
        let x = rand % map_dems.0;
        let y = if rand < map_dems.0 {
            0
        } else {
            map_dems.1
        };
        (x, y)
    } else {
        let rand = rand - map_dems.0 * 2;
        let y = rand % map_dems.1;
        let x = if rand < map_dems.1 {
            0
        } else {
            map_dems.0
        };
        (x, y)
    };

    spawn_enemy(commands, assets, enemy, Vec2::new(x as f32 - ((map_dems.0 as f32 + 1.0) / 2.0), y as f32 - ((map_dems.1 as f32 + 1.0) / 2.0)), round);
}