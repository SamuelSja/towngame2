


use bevy::prelude::*;

use super::{components::*, styles::*};

pub fn spawn_gui (
    mut coms: Commands,
    assets: Res<AssetServer>,
) {
    coms.spawn((
        main_style(),
        TopBar {},
    )).with_children(|parent| {
        parent.spawn((
            GoldText {},
            Text(String::from("Gold: #")),
        ));
    });
}







