use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::platformer::components::Item;


#[derive(Resource)]
pub struct Inventory {
    pub inventory: Vec<Item>
}

