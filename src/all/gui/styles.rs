


use bevy::prelude::*;


pub fn main_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        // justify_content: Content::Center,
        ..default()
    }
}

// pub fn top_bar_style() -> Node {
//     Node {
//         width: Val::Percent(100.0),
//         height: Val::Percent(20.0),
//         justify_content: JustifyContent::Center,
//         // align_items: AlignItems::Center,
//         ..default()
//     }
// }






