use bevy::{input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*, window::PrimaryWindow};

use crate::{all::{asset_consts::{SELECT_SOUNDS, SELL_SOUNDS, UPGRADE_BUILDING_SOUNDS}, buildings::{components::{BaseBuilding, Building}, systems::{at, building_at, comp_at, comp_at_mut, entity_at}}, helper::play_sounds, resources::Gold}, building::{self, clear_highlight, components::Highlight, gui::SCROLL_DIST, helper::{self, mouse_tile}, resources::{SelectType, Selected}}};
use super::{components::*, insert_labels, insert_shop_data, insert_shop_data_level};

pub fn scroll (
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut scrollable_q: Query<(&mut ScrollPosition, &Scrollable)>,
    window_q: Query<&Window, With<PrimaryWindow>>,

) {


    for event in mouse_wheel_events.read() {
        if let Ok(window) = window_q.get_single() {
            if let Some(mouse_pos) = window.cursor_position() {
                for (mut pos, scrollable) in scrollable_q.iter_mut() {

                    // Note: Check bounds
                    let mouse_par = mouse_pos / window.size();

                    let end_par = scrollable.start_par + scrollable.size_par;


                    if ! (scrollable.start_par.x <= mouse_par.x && mouse_par.x < end_par.x
                     && scrollable.start_par.y <= mouse_par.y && mouse_par.y < end_par.y) {
                        continue;
                    }


                    let dist = {
                        match event.unit {
                            MouseScrollUnit::Line => event.y * SCROLL_DIST,
                            MouseScrollUnit::Pixel => event.y,
                        }
                    };


                    pos.offset_y -= dist;
                }
            }
        }
    }
}

pub fn select (
    mut coms: Commands,
    buttons_q: Query<(&ShopBuilding, &Interaction), (Changed<Interaction>, With<ShopButton>)>,
    highlights_q: Query<Entity, With<Highlight>>,
    mut selected: ResMut<Selected>,
    assets: Res<AssetServer>,
) {
    for (building, interaction) in buttons_q.iter() {
        match interaction {
            Interaction::Pressed => {
                selected.val = SelectType::Placing(building.building.clone());
                clear_highlight(&mut coms, &highlights_q);
                println!("select");
                coms.spawn((
                    AudioPlayer::new(assets.load(SELECT_SOUNDS[0])),
                    PlaybackSettings::DESPAWN,
                ));
            },
            _ => {}
        }
    }
}

pub fn test_selected (
    cur_selected: Res<Selected>,
) {
    println!("cur_selected: {:?}", cur_selected.val)
}



pub fn test_scroll (
    scrollable_q: Query<&ScrollPosition, With<Scrollable>>,
) {
    if let Ok(scrollable) = scrollable_q.get_single() {
        println!("scrollable: {scrollable:#?}")
    } 
}

pub fn info_when_hover (
    mut coms: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<&Transform, With<Camera>>,
    building_q: Query<(&Transform, &BaseBuilding)>,
    mut image_q: Query<&mut ImageNode, With<InfoImage>>,
    text_q: Query<Entity, With<InfoText>>,
    info_q: Query<Entity, With<PlacedInfo>>,
    selected: Res<Selected>,
    assets: Res<AssetServer>,
) {
    let mut display_info = |tiles: &Vec<Vec2>| {

        if let Ok(info) = info_q.get_single() {
            let mut entity = coms.get_entity(info).unwrap();
            entity.despawn_descendants();

            for tile in tiles.iter() {
                if let Some(base) = comp_at(&building_q, &tile) {

                    let building = base.building.clone();

                    entity.with_children(|parent| {
                        insert_shop_data_level(parent, &assets, building, base.level);
                    });



                }



                


                // if let Ok(mut image) = image_q.get_single_mut() {
                //     image.image = building.texture(&assets);
                // }

                // if let Ok(entity) = text_q.get_single() {
                //     let mut e_coms = coms.get_entity(entity).unwrap();

                //     e_coms.despawn_descendants();

                //     if let Some(base) = base {
                //         e_coms.with_children(|parent| {
                //             insert_labels(parent, building.labels(base.level));
                //         });
                //     }
                // }

            }
        }
    };



    if let SelectType::Placed(tiles) = &selected.val {
        display_info(tiles);
    } else if let (Ok(window), Ok(camera)) = (window_q.get_single(), camera_q.get_single()) {
        if let Some(tile) = mouse_tile(window, camera) {
            display_info(&vec![tile]);
        }
    }
}

pub fn sell (
    mut coms: Commands,
    buildings_q: Query<(&Transform, &BaseBuilding, Entity)>,
    buttons_q: Query<&Interaction, (Changed<Interaction>, With<SellButton>)>,
    highlights_q: Query<Entity, With<Highlight>>,
    mut selected: ResMut<Selected>,
    mut gold: ResMut<Gold>,
    assets: Res<AssetServer>,
) {
    for interaction in buttons_q.iter() {
        match interaction {
            Interaction::Pressed => {
                if let SelectType::Placed(tiles) = &selected.val {

                    if ! tiles.is_empty() {
                        play_sounds(&mut coms, &assets, SELL_SOUNDS);
                    }

                    for tile in tiles {
                        helper::sell(&mut coms, &buildings_q, &mut gold, *tile);
                    }

                    selected.val = SelectType::None;
                    clear_highlight(&mut coms, &highlights_q);

                }
            } 
            _ => { 
                // Todo: hover and none
            }
        }
    }
}

pub fn upgrade (
    mut coms: Commands,
    mut buildings_q: Query<(&Transform, &mut BaseBuilding)>,
    buttons_q: Query<&Interaction, (Changed<Interaction>, With<UpgradeButton>)>,
    selected: ResMut<Selected>,
    mut gold: ResMut<Gold>,
    assets: Res<AssetServer>,

) {
    for interaction in buttons_q.iter() {
        match interaction {
            Interaction::Pressed => {
                if let SelectType::Placed(tiles) = &selected.val {



                    let mut buildings = Vec::new();
                    let mut cost = 0;

                    for (transform, base) in buildings_q.iter_mut() {
                        for tile in tiles {
                            if at(transform, tile) {
                                if let Some(build_cost) = base.building.cost(base.level) {
                                    cost += build_cost;
                                }

                                buildings.push(base);
                                break;
                            }
                        }
                    }

                    // let buildings = tile.iter().map(|tile| {
                    //     comp_at_mut(&mut buildings_q, tile)
                    // });

                    
                    if gold.val >= cost {
                        gold.val -= cost;

                        for building in buildings.iter_mut() {
                            building.level_up(1);

                        }

                        play_sounds(&mut coms, &assets, UPGRADE_BUILDING_SOUNDS);
                    } 
                }
            },
            _ => { 
                // Todo: hover and none
            }
        }
    }
}

pub fn update_upgrade_text (
    buildings_q: Query<(&Transform, &BaseBuilding)>,
    mut text_q: Query<&mut Text, With<UpgradeText>>,
    selected: ResMut<Selected>,
) {
    if let Ok(mut text) = text_q.get_single_mut() {
        if let SelectType::Placed(pos) = &selected.val {
            let building = comp_at(&buildings_q, &pos[0]).expect("Should only have tile if building on tile");

            if let Some(cost) = building.building.cost(building.level) {
                text.0 = format!("Upgrade\nCost: {}", cost);
            }
        }
    } 
}

pub fn set_sell (
    button_q: Query<&Interaction, (With<SetSellButton>, Changed<Interaction>)>,
    mut selected: ResMut<Selected>,
) {
    if let Ok(button) = button_q.get_single() {
        match button {
            Interaction::Pressed => {
                selected.val = SelectType::Selling;
            },
            _ => {
                // Todo: hover and none
            }
        }
    }
}

