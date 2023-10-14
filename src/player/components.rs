use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Copy, Clone)]
pub struct Item {
    pub item_type: ItemType,
}

impl Default for Item {
    fn default() -> Self {
        Self { item_type: ItemType::PickUp }
    }
}

#[derive(Copy, Clone)]
pub enum ItemType {
    PickUp
}

#[derive(Resource)]
pub struct Inventory {
    pub inventory: Vec<Item>
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ItemBundle {
    #[sprite_bundle("cup.png")]
    pub sprite_bundle: SpriteBundle,
    pub item: Item,
}