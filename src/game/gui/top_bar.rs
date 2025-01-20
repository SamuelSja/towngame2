

use bevy::prelude::*;

use crate::{all::gui::components::TopBar, game::resources::Round};

#[derive(Component)]
pub struct RoundLabel;

pub fn insert_round_label (
    mut coms: Commands,
    bar_q: Query<Entity, With<TopBar>>, 
) {
    if let Ok(bar) = bar_q.get_single() {
        coms.get_entity(bar).unwrap().with_child((
            Node {
                ..default()
            },
            RoundLabel {},
            Text::new("Round: #"),
        ));
    }
}

pub fn remove_round_label (
    mut coms: Commands,
    label_q: Query<Entity, With<RoundLabel>>,
) {
    if let Ok(label) = label_q.get_single() {
        coms.get_entity(label).unwrap().despawn();
    }
}


pub fn update_round_label (
    mut label_q: Query<&mut Text, With<RoundLabel>>,
    round: ResMut<Round>,
) {
    if let Ok(mut label) = label_q.get_single_mut() {
        label.0 = format!("Round: {}", round.val);
    }
}




















