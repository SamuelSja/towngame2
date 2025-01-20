

use bevy::{math::VectorSpace, prelude::*};

use super::{resources::PrevState, states::AppState, BUILDABLE_SIZE, CAMERA_MOVE_SPEED, GROUND_IMAGE_1, GROUND_IMAGE_2, TILE_SIZE};



pub fn spawn_camera (
    mut commands: Commands,
) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        }
    ));
}

pub fn move_camera (
    mut camera_q: Query<&mut Transform, With<Camera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut camera) = camera_q.get_single_mut() {

        let mut direction = Vec2::ZERO;

        if keyboard.pressed(KeyCode::KeyD) {
            direction.x += 1.;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            direction.x -= 1.;
        }
        if keyboard.pressed(KeyCode::KeyW) {
            direction.y += 1.;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction.y -= 1.;
        }

        let camera_change = direction.normalize_or_zero() * CAMERA_MOVE_SPEED;

        camera.translation += Vec3::new(camera_change.x, camera_change.y, 0.); 
    }
}

pub fn spawn_ground (
    mut coms: Commands,
    assets: Res<AssetServer>,
) {

    for i in 0..(BUILDABLE_SIZE.0 + 10) {
        let x_pos = (i as i32 - (BUILDABLE_SIZE.0 as i32 + 10) / 2) as f32 * TILE_SIZE.x;

        for j in 0..(BUILDABLE_SIZE.1 + 10) {
            let y_pos = (j as i32 - (BUILDABLE_SIZE.1 as i32 + 10) / 2) as f32 * TILE_SIZE.y;

            coms.spawn((
                Sprite {
                    image: assets.load(GROUND_IMAGE_2),
                    ..default()
                },
            ))
            .insert(Transform::from_xyz(x_pos, y_pos, -2.0));
        }

    }

    for i in 0..BUILDABLE_SIZE.0 {
        let x_pos = (i as i32 - BUILDABLE_SIZE.0 as i32 / 2) as f32 * TILE_SIZE.x;

        for j in 0..BUILDABLE_SIZE.1 {
            let y_pos = (j as i32 - BUILDABLE_SIZE.1 as i32 / 2) as f32 * TILE_SIZE.y;

            coms.spawn((
                Sprite {
                    image: assets.load(GROUND_IMAGE_1),
                    ..default()
                },
            ))
            .insert(Transform::from_xyz(x_pos, y_pos, -1.0));
        }

    }

    // coms.spawn((
    //     Sprite {
    //         image: GROUND_IMAGE_1,
    //         ..default() 
    //     }
    // ))
}


pub fn test_switch_states (
    keyboard: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyY) {
        app_state.set(AppState::Menu);
    } else if keyboard.just_pressed(KeyCode::KeyU) {
        app_state.set(AppState::Build);
    } else if keyboard.just_pressed(KeyCode::KeyI) {
        app_state.set(AppState::Game);
    } else if keyboard.just_pressed(KeyCode::KeyO) {
        app_state.set(AppState::GameOver);
    }
}


// Todo: It is possible for the state to switch twice before this updates
pub fn update_prev_state (
    mut prev_state: ResMut<PrevState>,
    state: Res<State<AppState>>,
) {
    if *state != prev_state.1 {
        println!("update prev state");
        prev_state.0 = Some(prev_state.1.clone());
        prev_state.1 = state.clone()
    } 
}
