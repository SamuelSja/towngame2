

use bevy::prelude::*;
use crate::all::TILE_SIZE;

use super::{resources::PrevState, states::AppState, update_prev_state};




pub fn intersect_1d(start1: f32, end1: f32, start2: f32, end2: f32) -> bool {
    start1 <= start2 && start2 <= end1
    || start2 <= start1 && start1 <= end2
    || start1 <= end2 && end2 <= end1
    || start2 <= end1 && end1 <= end2
}

pub fn intersect_2d(start1: Vec2, end1: Vec2, start2: Vec2, end2: Vec2) -> (bool, bool) {
    let x = intersect_1d(start1.x, end1.x, start2.x, end2.x);
    let y = intersect_1d(start1.y, end1.y, start2.y, end2.y);

    (x, y)
}

pub fn collide(pos1: Vec3, size1: Vec2, pos2: Vec3, size2: Vec2) -> (Option<f32>, Option<f32>) {
    let pos1 = Vec2::new(pos1.x, pos1.y);
    let pos2 = Vec2::new(pos2.x, pos2.y);

    let start1 = pos1 - size1 / 2.0;
    let end1 = pos1 + size1 / 2.0;
    let start2 = pos2 - size2 / 2.0;
    let end2 = pos2 + size2 / 2.0;

    let data = intersect_2d(start1, end1, start2, end2);

    let mut change_x = None;
    if data.0 {
        let negative = pos1.x < pos2.x;
        change_x = Some(if negative {
            -(end1.x - start2.x)
        } else {
            end2.x - start1.x
        });
    }

    let mut change_y = None;
    if data.1 {
        let negative = pos1.y < pos2.y;
        change_y = Some(if negative {
            -(end1.y - start2.y)
        } else {
            end2.y - start1.y
        });
    }
    
    if change_x.is_some() && change_y.is_some() {
        if change_x.unwrap().abs() < change_y.unwrap().abs() {
            change_y = Some(0.0);
        } else {
            change_x = Some(0.0);
        }
        (change_x, change_y)
    } else {
        (None, None)
    }
}

pub fn did_collide(pos1: Vec3, size1: Vec2, pos2: Vec3, size2: Vec2) -> bool {
    let collide = collide(pos1, size1, pos2, size2);

    if let (None, None) = collide {
        false
    } else {
        true
    }
}

pub fn restrict_transform_movement(
    moving_obj: &mut Transform,
    static_obj: &Transform,
) {
    let collide = collide(
        moving_obj.translation,
        TILE_SIZE * 0.9,
        static_obj.translation,
        TILE_SIZE,
    );

    if let (Some(x), _) = collide {
        moving_obj.translation.x += x;
    }
    if let (_, Some(y)) = collide {
        moving_obj.translation.y += y;
    }
}


pub fn exit_either<M, S1: States, S2: States>(app: &mut App, states: (S1, S2), system: impl IntoSystemConfigs<M> + Copy) -> &mut App {
    app.add_systems(OnExit(states.0.clone()), system.run_if (
        in_state(states.1.clone()).map(|val| ! val)
    ))
    .add_systems(OnExit(states.1), system.run_if (
        in_state(states.0).map(|val| ! val)
    ))
}

pub fn enter_either<M>(app: &mut App, states: (AppState, AppState), system: impl IntoSystemConfigs<M> + Copy) -> &mut App {
    let states1 = states.clone();
    let states2 = states.clone();

    app.add_systems(OnEnter(states.1.clone()), system.run_if (
        move |prev_state: Res<PrevState>| {
            let states = states1.clone();
            prev_state.1 != states.0
        }
    ))
    .add_systems(OnEnter(states.0.clone()), system.run_if (
        move |prev_state: Res<PrevState>| {
            let states = states2.clone();
            prev_state.1 != states.1
        }
    ))
}