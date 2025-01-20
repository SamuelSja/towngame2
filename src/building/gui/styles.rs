
use bevy::prelude::*;



pub const LEFT_BAR_PAR: f32 = 0.2;


pub fn main_style() -> Node {
    Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::FlexEnd,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    }
}

pub fn middle_style() -> Node {
    Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::SpaceBetween,
        width: Val::Percent(100.0),
        height: Val::Percent(20.0),
        ..default()
    }
}

pub fn vertical_bar_style() -> Node {
    Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        width: Val::Percent(LEFT_BAR_PAR * 100.0),
        height: Val::Percent(100.0),
        overflow: Overflow { x: OverflowAxis::Clip, y: OverflowAxis::Scroll },
        ..default() 
    }
}

pub fn shop_item_style() -> Node {
    Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        width: Val::Percent(90.0),
        // height: Val::Px(1000.0),
        margin: UiRect::all(Val::Percent(5.0)),
        ..default()
    }
}

pub fn shop_image_style() -> Node {
    Node {
        width: Val::Px(50.0),
        height: Val::Px(50.0),
        margin: UiRect::horizontal(Val::Auto),
        ..default()
    }
}

pub fn shop_label_row_style() -> Node {
    Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        width: Val::Percent(100.0),
        height: Val::Percent(40.0),
        ..default()
    }
}

pub fn shop_label_style() -> Node {
    Node {
        width: Val::Percent(50.0),
        height: Val::Percent(100.0),
        ..default()
    }
}



pub fn button_style() -> Node {
    Node {
        width: Val::Percent(80.0),
        height: Val::Percent(5.0),
        ..default()
    }
}

pub fn info_text_style() -> Node {
    Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        height: Val::Percent(90.0),
        overflow: Overflow { x: OverflowAxis::Clip, y: OverflowAxis::Scroll },
        ..default() 
    }
}













