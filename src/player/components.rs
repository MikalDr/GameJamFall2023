use bevy::prelude::*;

use crate::platformer::components::ItemType;


#[derive(Resource, Clone, Default)]
pub struct Inventory {
    pub inventory: Vec<ItemType>
}



#[derive(Resource)]
pub struct JumpScareTime(pub Timer);



#[derive(Resource)]
pub struct JumpScareEventTimer(pub Timer);