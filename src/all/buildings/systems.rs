

use bevy::{color::palettes::css::*, ecs::query::{QueryData, WorldQuery}, prelude::*};

use crate::all::TILE_SIZE;

use super::components::{BaseBuilding, BaseBuildingGame, Building, HealthBarBack, HealthBarFront};


// /// Note: This is O(n) and therefore should be improved if it is needed every frame
// pub fn items_at<T: QueryData> (
//     building_q: &Query<(&Transform, T)>,
//     pos: &Vec2,
// ) -> Option<T> {
// 
//     for test1 in building_q.iter() {
//         let (building, data): (&Transform, T) = test1;
//         // let test: T = data;
// 
//         let translation = building.translation;
// 
//         let mut translation = Vec2::new(translation.x / TILE_SIZE.x, translation.y / TILE_SIZE.y).round();
//         translation = translation.round();
// 
// 
//         if translation.x as i32 == pos.x as i32 && translation.y as i32 == pos.y as i32 {
//             // let test: T = data;
//             // let ans: Option<T> = Some(data);
//         }
//     }
// 
//     // Building::None
//     None
// }


/// Finds the thing at the thing
/// 
/// pos is the tile location, not pixel location
pub fn at (
    transform: &Transform,
    pos: &Vec2,
) -> bool {
    let translation = transform.translation;

    let mut translation = Vec2::new(translation.x / TILE_SIZE.x, translation.y / TILE_SIZE.y).round();
    translation = translation.round();


    return translation.x as i32 == pos.x as i32 && translation.y as i32 == pos.y as i32;

}


/// Note: This is O(n) and therefore should be improved if it is needed every frame
/// 
/// pos is the tile location, not pixel location
pub fn entity_at (
    building_q: &Query<(&Transform, Entity), With<BaseBuilding>>,
    pos: &Vec2,
) -> Option<Entity> {
    for (building, entity) in building_q.iter() {

        // let translation = building.translation;

        // let mut translation = Vec2::new(translation.x / TILE_SIZE.x, translation.y / TILE_SIZE.y).round();
        // translation = translation.round();


        // if translation.x as i32 == pos.x as i32 && translation.y as i32 == pos.y as i32 {
        if at(building, pos) {
            return Some(entity);
        }
    }

    None
}



/// Note: This is O(n) and therefore should be improved if it is needed every frame
/// 
/// pos is the tile location, not pixel location
pub fn building_at (
    building_q: &Query<(&Transform, &BaseBuilding)>,
    pos: &Vec2,
) -> Building {
    for (building, base) in building_q.iter() {

        // let translation = building.translation;

        // let mut translation = Vec2::new(translation.x / TILE_SIZE.x, translation.y / TILE_SIZE.y).round();
        // translation = translation.round();


        // if translation.x as i32 == pos.x as i32 && translation.y as i32 == pos.y as i32 {
        if at(building, pos) {
            return base.building.clone();
        }
    }

    Building::None
}

/// Note: This is O(n) and therefore should be improved if it is needed every frame.
/// 
/// Returns None when the there is no component at the location.
/// 
/// pos is the tile location, not pixel location
pub fn comp_at<'a, C: Component> (
    building_q: &'a Query<(&Transform, &C)>,
    pos: &Vec2,
) -> Option<&'a C> {
    for (building, comp) in building_q.iter() {

        // let translation = building.translation;

        // let mut translation = Vec2::new(translation.x / TILE_SIZE.x, translation.y / TILE_SIZE.y).round();
        // translation = translation.round();


        // if translation.x as i32 == pos.x as i32 && translation.y as i32 == pos.y as i32 {
        if at(building, pos) {
            return Some(comp);
        }
    }
    None
}

/// Gets the component at the position as mutable
///
/// Note: This is O(n) and therefore should be improved if it is needed every frame
/// 
/// pos is the tile location, not pixel location
pub fn comp_at_mut<'a, C: Component> (
    building_q: &'a mut Query<(&Transform, &mut C)>,
    pos: &Vec2,
) -> Option<Mut<'a, C>> {
    for (building, comp) in building_q.iter_mut() {

        let translation = building.translation;

        let mut translation = Vec2::new(translation.x / TILE_SIZE.x, translation.y / TILE_SIZE.y).round();
        translation = translation.round();


        if translation.x as i32 == pos.x as i32 && translation.y as i32 == pos.y as i32 {
            return Some(comp);
        }
    }
    None
}




/// Note: This is O(n) and therefore should be improved if it is needed every frame
/// 
/// pos is the tile location, not pixel location
pub fn is_free (
    building_q: &Query<(&Transform, &BaseBuilding/*, Entity*/)>,
    pos: &Vec2,
) -> bool {
    for (transform, base/*, entity*/) in building_q {
        if at(transform, pos) {
            return false;
        }
    }

    true
}

/// Note: This is O(n) and therefore should be improved if it is needed every frame
/// 
/// pos is the tile location, not pixel location
pub fn is_free_entity (
    building_q: &Query<(&Transform, &BaseBuilding, Entity)>,
    pos: &Vec2,
) -> bool {
    for (transform, base, entity) in building_q {
        if at(transform, pos) {
            return false;
        }
    }

    true
}



/// Adds the health bar to a new building
pub fn insert_health_bar(entity: &mut EntityCommands, assets: &Res<AssetServer>) {
    entity.with_children(|parent| {
        parent.spawn((
            Sprite {
                image: assets.load("images/blank.png"),
                color: BLACK.into(),
                ..default()
            },
            HealthBarBack {},
        ));

        parent.spawn((
            Sprite {
                image: assets.load("images/blank.png"),
                color: GREEN.into(),
                ..default()
            },
            HealthBarFront {},
        ));

    });
}

/// Updates the health bars on the buildings
/// 
/// Note: &Transform in building_q is not necessary, so I should rewrite
pub fn update_health_bar (
    mut back_q: Query<&mut Transform, With<HealthBarBack>>,
    mut front_q: Query<&mut Transform, (With<HealthBarFront>, Without<HealthBarBack>)>,
    building_q: Query<(&Transform, &Children, &BaseBuilding), (With<BaseBuilding>, Without<HealthBarFront>, Without<HealthBarBack>)>,
    building_game_q: Query<&BaseBuildingGame>,
) {
    for (_build_transform, children, base) in building_q.iter() {

        let y_scale = TILE_SIZE.y / 5.0;
        let y_translation = -(TILE_SIZE.y / 2.0) + (TILE_SIZE.y / 10.0);

        for &child in children.iter() {
            if let Ok(mut back_transform) = back_q.get_mut(child) {
                back_transform.scale.x = TILE_SIZE.x * 0.8;
                back_transform.scale.y = y_scale;
                back_transform.translation.x = 0.0;
                back_transform.translation.y = y_translation;
                back_transform.translation.z = 0.1;
            }

            if let Ok(mut front_transform) = front_q.get_mut(child) {
                for &child in children.iter() {
                    if let Ok(building_game) = building_game_q.get(child) {
                        let size = (TILE_SIZE.x * 0.8) * (building_game.health / (base.max_health)(base.level));
                        front_transform.scale.x = size;
                        front_transform.scale.y = y_scale;
                        front_transform.translation.x = -(((TILE_SIZE.x * 0.8) - size) / 2.0);
                        front_transform.translation.y = y_translation;
                        front_transform.translation.z = 0.2;
                    }
                }
            }
        }
    }
}