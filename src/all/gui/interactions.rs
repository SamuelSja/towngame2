


use bevy::prelude::*;

use crate::all::resources::Gold;

use super::components::GoldText;

pub fn update_gold (
    mut label: Query<&mut Text, With<GoldText>>,
    gold: Res<Gold>,
) {
    if let Ok(mut text) = label.get_single_mut() {
        text.0 = format!("Gold: {}", gold.val);
    }
}













