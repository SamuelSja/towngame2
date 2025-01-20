

pub mod interaction;
pub mod layout;
pub mod styles;
pub mod components;

use bevy::prelude::*;

use interaction::*;
use layout::*;

use crate::all::states::AppState;

pub const SCROLL_DIST: f32 = 50.0;






pub struct GUIPlug;
impl Plugin for GUIPlug {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Build), spawn_gui)
        .add_systems(OnExit(AppState::Build), despawn_gui)
        .add_systems(Update, (
            // test_scroll,
            // test_selected,
            upgrade,
            sell,
            info_when_hover,
            select,
            scroll,
            update_upgrade_text,
            set_sell,
        ).run_if(in_state(AppState::Build)))
        ;
    }
}
