use bevy::prelude::*;
use bevy_ecs_ldtk::EntityIid;

use crate::player::Player;

#[derive(Component, Debug, Clone, Default)]
//TODO: Need to put this on something
pub struct Inventory {
    // An array with maximum of 10 effects
    pub items: [InventoryEntry; 10],
}
#[derive(Default, Debug, Clone)]
pub struct InventoryEntry {
    name: String,
    status_effect: StatusEffect,
}
#[derive(Default, PartialEq, Debug, Clone)]
pub enum StatusEffect {
    #[default]
    None,
    Effect1,
    Effect2,
    Effect3
}

pub fn remove_effect() {}

pub fn add_effect(
    commands: Commands,
    key_input: Res<Input<KeyCode>>,
    mut player_inventory_query: Query<&mut Inventory, With<Player>>
) 
{
}