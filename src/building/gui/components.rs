
use bevy::prelude::*;

use crate::all::buildings::components::Building;


#[derive(Component)]
pub struct ShopButton;

#[derive(Component)]
pub struct Scrollable {
    pub start_par: Vec2,
    pub size_par: Vec2,
}

#[derive(Component)]
pub struct ShopBuilding {
    pub building: Building
}

#[derive(Component)]
pub struct MainNode;

#[derive(Component)]
pub struct BuildingInfo;

#[derive(Component)]
pub struct InfoImage;

#[derive(Component)]
pub struct InfoText;

#[derive(Component)]
pub struct SellButton;

#[derive(Component)]
pub struct UpgradeButton;

#[derive(Component)]
pub struct UpgradeText;








#[derive(Component)]
pub struct SetSellButton;


#[derive(Component)]
pub struct PlacedInfo;












