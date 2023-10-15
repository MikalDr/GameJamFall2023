use bevy::prelude::*;

use crate::platformer::components::ItemType;


#[derive(Resource, Clone, Default)]
pub struct Inventory {
    pub inventory: Vec<ItemType>
}

