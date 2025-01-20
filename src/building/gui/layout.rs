

use bevy::{color::palettes::css::*, prelude::*};

use crate::all::buildings::components::Building;

use super::{components::*, styles::*};


pub fn despawn_gui (
    mut commands: Commands,
    main_node_q: Query<Entity, With<MainNode>>,
) {
    if let Ok(entity) = main_node_q.get_single() {
        commands.get_entity(entity).unwrap().despawn_recursive();
    }
}

pub fn spawn_gui (
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let spawn_shop_item = |parent: &mut ChildBuilder<'_>, building: Building| {
        parent.spawn(
            shop_item_style()
        )
        .insert(Button {})
        .insert(ShopButton {})
        .insert(ShopBuilding { building: building.clone() })
        .insert(BackgroundColor(ORANGE.into()))
        .with_children(|parent| {
            insert_shop_data(parent, &assets, building);
        });
    };

    commands.spawn(
        main_style()
    )
    .insert(MainNode {})
    .with_children(|parent| {
        parent.spawn(
            vertical_bar_style()
        ).insert(BackgroundColor(BLUE.into()))
        .insert(Scrollable {
            start_par: Vec2::new(0.0, 0.0),
            size_par: Vec2::new(LEFT_BAR_PAR, 100.0),
        })
        .with_children(|parent| {
            spawn_shop_item(parent, Building::Town);
            spawn_shop_item(parent, Building::Archer);
            spawn_shop_item(parent, Building::Wall);
            spawn_shop_item(parent, Building::Town);
            spawn_shop_item(parent, Building::Archer);
            spawn_shop_item(parent, Building::Wall);
            spawn_shop_item(parent, Building::Town);
            spawn_shop_item(parent, Building::Archer);
            spawn_shop_item(parent, Building::Wall);
        });

        parent.spawn((
            middle_style(),
        ))
        .with_children(|parent| {
            parent.spawn((
                button_style(),
                Button {},
                SetSellButton {},
                Text::new("Sell"),
            ));
        });

        parent.spawn(
            vertical_bar_style()
        ).insert((
            BackgroundColor(PURPLE.into()),
            BuildingInfo {},
        )).with_children(|parent| {
            parent.spawn((
                info_text_style(),
                PlacedInfo {},
                Scrollable {
                    start_par: Vec2::new(1.0 - LEFT_BAR_PAR, 0.0),
                    size_par: Vec2::new(LEFT_BAR_PAR, 100.0),
                },

            ));
            // parent.spawn((
            //     shop_image_style(),
            //     InfoImage {},
            //     ImageNode {
            //         ..default()
            //     }
            // ));

            // parent.spawn((
            //     info_text_style(),
            //     InfoText {},
            // ));

            parent.spawn((
                button_style(),
                SellButton {},
                Button {},
            )).with_children(|parent| {
                parent.spawn((
                    Node::default(),
                    // button_style(),
                    Text::new("Sell"),
                ));
            });

            parent.spawn((
                button_style(),
                UpgradeButton {},
                Button {},
            )).with_children(|parent| {
                parent.spawn((
                    // button_style(),
                    Node::default(),
                    Text::new("Upgrade"),
                    UpgradeText {},
                ));
            });


        });
        
    })



    ;
}




/// Adds the building's image and labels to the ChildBuilder
/// 
/// This will assume level 1
pub fn insert_shop_data(parent: &mut ChildBuilder<'_>, assets: &Res<AssetServer>, building: Building) {

    insert_shop_data_level(parent, assets, building, 1);

    // insert_image(parent, &assets, building.clone());

    // parent.spawn(
    //     shop_label_style()
    // ).insert(
    //     Text::new(building.name()),
    // );

    // let labels = building.labels(1);

    // insert_labels(parent, labels);
}

pub fn insert_shop_data_level(parent: &mut ChildBuilder<'_>, assets: &Res<AssetServer>, building: Building, level: u32) {
    // parent.spawn(
    //     shop_image_style()
    // ).insert(
    //     ImageNode {
    //         image: building.texture(&assets),
    //         ..default()
    //     }
    // );
    insert_image(parent, &assets, building.clone());

    parent.spawn(
        shop_label_style()
    ).insert(
        Text::new(building.name()),
    );

    let labels = building.labels(level);

    insert_labels(parent, labels);
}


/// Add the building's image to the ChildBuilder
pub fn insert_image(parent: &mut ChildBuilder<'_>, assets: &Res<AssetServer>, building: Building) {
    parent.spawn(
        shop_image_style()
    ).insert(
        ImageNode {
            image: building.texture(assets),
            ..default()
        }
    );
}

/// Adds the labels to the ChildBuilder
pub fn insert_labels(parent: &mut ChildBuilder, labels: Vec<(String, f32)>) {
    for i in 0..(labels.len() / 2) {
        parent.spawn(
            shop_label_row_style()
        ).with_children(|parent| {
            for j in 0..=1 {
                if let Some((label, value)) = &labels.get(i * 2 + j) {
                    parent.spawn(
                        shop_label_style()
                    ).insert(
                        Text::new(format!("{}: {}", label, value))
                    );
                }

            }
        });
    }

    if labels.len() % 2 == 1 {

        let (name, val) = &labels[labels.len() - 1];

        parent.spawn(shop_label_row_style())
        .with_children(|parent| {
            parent.spawn(shop_label_style())
            .insert(Text::new(format!("{name}: {val}")));
        });
    }

}