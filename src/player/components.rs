use bevy::prelude::*;

use crate::platformer::components::ItemType;


#[derive(Resource)]
pub struct Inventory {
    pub inventory: Vec<ItemType>
}

