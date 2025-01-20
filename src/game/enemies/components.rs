

use bevy::prelude::*;

use crate::all::buildings::components::BuildingComponent;


type Number = f32;


// #[derive(Component)]
// pub struct Enemy {
//     pub health: f32,
// }

// Note: Could make this DRYer by making a trait with Enemy and Building that has one or two methods to over ride (components), then the others are defined using those
#[derive(Clone)]
pub enum Enemy {
    Normal,
}

impl Enemy {
    pub fn spawn(&self, entity: &mut EntityCommands, level: u32) {
        for component in self.components(level) {
            component.insert(entity)
        }
    }

    pub fn spawn_new<'a>(&self, coms: &'a mut Commands, level: u32) -> EntityCommands<'a> {
        let mut entity: EntityCommands<'a> = coms.spawn_empty();
        self.spawn(&mut entity, level);
        entity
    }

    pub fn texture(&self, assets: &Res<AssetServer>) -> Handle<Image> {
        assets.load(
            match self {
                Self::Normal => "images/enemies/enemy1.png",
                _ => "todo.png",
            }
        )
    }

    pub fn name(&self) -> String {
        match self {
            Self::Normal => "Normal"
        }.to_string()
    }

    pub fn labels(&self, round: u32) -> Vec<(String, Number)> {
        self.components(round).into_iter().map(|component| {
            component.get_data()
        }).flatten().collect()
    }

    pub fn components(&self, round: u32) -> Vec<EnemyComponent> {
        let components = match self {
            Self::Normal => {vec![
                EnemyComponent::Move { speed: 1.0 * (1.05_f32).powf(round as f32) },
                EnemyComponent::Solid,
                EnemyComponent::Attack { damage: 60.0 * (1.1_f32).powf(round as f32), reload: 1.0 },
                EnemyComponent::Base { health: 50.0 * (1.1_f32).powf(round as f32), enemy: self.clone() },
                EnemyComponent::Drops { gold: /*(1.0 * (1.1_f32).powf(round as f32)) as u32*/ round },
            ]},
        };

        components
    }
}



pub enum EnemyComponent {
    Base { health: f32, enemy: Enemy },
    Move { speed: f32 },
    Attack { damage: f32, reload: f32 },
    Solid,
    Drops { gold: u32 },
}

impl EnemyComponent {
    pub fn get_data(&self) -> Vec<(String, Number)> {
        match self {
            Self::Base { health, enemy: _ } => {vec![
                ( String::from("Health"), *health )
            ]},
            Self::Move { speed } => {vec![
                ( String::from("Move"), *speed )
            ]},
            Self::Attack { damage, reload } => {vec![
                ( String::from("Damage"), *damage ),
                (String::from("Reload"), *reload)
            ]}
            _ => Vec::new(),
        }
    }

    pub fn insert(&self, entity: &mut EntityCommands) {
        match self {
            Self::Base { health, enemy } => {
                entity.insert(Base {
                    health: *health,
                    enemy: enemy.clone(),
                })
            },
            Self::Move { speed } => {
                entity.insert(Move {
                    speed: *speed,
                })
            },
            Self::Attack { damage, reload } => {
                entity.insert(Attack {
                    damage: *damage,
                    reload: *reload,
                    cur_reload: *reload,
                })
            },
            Self::Solid => {
                entity.insert(Solid {
                })
            },
            Self::Drops { gold } => {
                entity.insert(Drops {
                    gold: *gold,
                })
            }
        };
    }
}


#[derive(Component)]
pub struct Base {
    pub health: f32,
    pub enemy: Enemy,
}

#[derive(Component)]
pub struct Move {
    pub speed: f32,
}

#[derive(Component)]
pub struct Attack {
    pub damage: f32,
    pub reload: f32,
    pub cur_reload: f32,
}

#[derive(Component)]
pub struct Solid;

#[derive(Component)]
pub struct Drops {
    pub gold: u32,
}