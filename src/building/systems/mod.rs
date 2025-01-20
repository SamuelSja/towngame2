
pub mod helper;

use bevy::{prelude::*, window::PrimaryWindow};
use helper::{mouse_tile, place_single_building, sell};

use crate::all::{asset_consts::{PLACE_SOUNDS, SELECT_SOUNDS}, buildings::{components::{BaseBuilding, Building, BuildingComponent}, systems::{building_at, entity_at, is_free, is_free_entity}}, helper::{play_sound, play_sounds}, resources::Gold, BUILDABLE_SIZE, TILE_SIZE};

use super::{components::Highlight, gui::styles::LEFT_BAR_PAR, resources::{SelectType, Selected}};



// Todo: this fn is a bit of a mess
pub fn clicked (
    mut coms: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<&Transform, With<Camera>>,
    buildings_q: Query<(&Transform, &BaseBuilding, Entity)>,
    highlights_q: Query<Entity, With<Highlight>>,
    mut mouse_buttons: ResMut<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    assets: Res<AssetServer>,
    mut selected: ResMut<Selected>,
    mut gold: ResMut<Gold>,
) {
    if let (Ok(window), Ok(camera)) = (window_q.get_single(), camera_q.get_single()) {
        let held = ! mouse_buttons.just_pressed(MouseButton::Left);
        let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
        if mouse_buttons.just_pressed(MouseButton::Left) || mouse_buttons.pressed(MouseButton::Left) {
            if let Some(mut mouse_pos) = window.cursor_position() {

                // Section: Check bounds
                if mouse_pos.x <= window.width() * LEFT_BAR_PAR {
                    return;
                }
            }
            if let Some(tile) = mouse_tile(window, camera) {
                if ! (-(BUILDABLE_SIZE.0 as f32) / 2.0 <= tile.x && tile.x < BUILDABLE_SIZE.1 as f32 / 2.0 && -(BUILDABLE_SIZE.1 as f32) / 2.0 <= tile.y && tile.y < BUILDABLE_SIZE.1 as f32 / 2.0) {
                    return;
                }

                match selected.val.clone() {
                    SelectType::Placing(building) => {
                        if ! is_free_entity(&buildings_q, &tile) {
                            if ! held {
                                selected.val = SelectType::Placed(vec![tile]);
                                select_building(&mut coms, &assets, tile);
                            }
                            return;
                        }
                        place_single_building(&mut coms, &assets, &building, tile, &mut gold);
                        play_sounds(&mut coms, &assets, PLACE_SOUNDS);
                    },
                    SelectType::Placed(mut pos) => {
                        if/* ! held &&*/ ! is_free_entity(&buildings_q, &tile) {
                            if ! shift && ! held {
                                // Note: Clear placed queue
                                pos.clear();
                                clear_highlight(&mut coms, &highlights_q);
                            }

                            // Note do select without shift when held
                            if shift || ! held {
                                if ! pos.contains(&tile) {
                                    pos.insert(0, tile);
                                    select_building(&mut coms, &assets, tile);
                                }
                            }

                            selected.val = SelectType::Placed(pos);
                        }
                    },
                    SelectType::Selling => {
                        // let building_entity = {
                        //     for (transform, base, entity) in buildings_q {
                        //         if at(transform, tile) {
                        //             entity
                        //         }
                        //     }
                        // }

                        sell(&mut coms, &buildings_q, &mut gold, tile);

                    },
                    SelectType::None => {
                        if ! is_free_entity(&buildings_q, &tile) {
                            if ! held {
                                selected.val = SelectType::Placed(vec![tile]);
                                select_building(&mut coms, &assets, tile);
                            }
                        }
                    }
                    _ => {}

                }
            }
        }
    }
}

/// pos is the tile location. highlight combined with play sounds
pub fn select_building(coms: &mut Commands, assets: &Res<AssetServer>, pos: Vec2) {
    highlight(coms, &assets, pos);
    play_sounds(coms, &assets, SELECT_SOUNDS);
}

/// pos is the tile location
pub fn highlight(coms: &mut Commands, assets: &Res<AssetServer>, pos: Vec2) {
    coms.spawn(
        (
            Sprite {
                image: assets.load("images/select.png"),
                ..default()
            },
            Transform::from_xyz(
                pos.x * TILE_SIZE.x,
                pos.y * TILE_SIZE.y,
                10.0,
            ),
            Highlight {},
        )
    );
}

pub fn clear_highlight(coms: &mut Commands, highlights_q: &Query<Entity, With<Highlight>>) {
    for entity in highlights_q.iter() {
        coms.get_entity(entity).unwrap().despawn();
    }
}



pub fn deselect_on_esc (
    mut coms: Commands,
    highlights_q: Query<Entity, With<Highlight>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<Selected>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        selected.val = SelectType::None;
        clear_highlight(&mut coms, &highlights_q);
    }
}

pub fn deselect (
    mut coms: Commands,
    mut selected: ResMut<Selected>,
    highlights_q: Query<Entity, With<Highlight>>
) {
    selected.val = SelectType::None;
    clear_highlight(&mut coms, &highlights_q);
}