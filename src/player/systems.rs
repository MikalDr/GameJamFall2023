use bevy::prelude::*;

use crate::platformer::{components::{Item, WinItem, WinCon, ItemType}, systems::{invert_controls, Controls}};

use super::{components::Inventory, Player};

#[derive(Resource)]
pub struct ActivePlayerEffects {
    pub invert : bool,
    pub continous_move: bool,
    pub rotating_world: bool,
    pub moon_gravity: bool,
}

pub fn pick_up_item(
    input: Res<Input<KeyCode>>,
    item_query: Query<(Entity, &GlobalTransform, &ItemType)>,
    player_query: Query<&GlobalTransform, With<Player>>,
    mut inventory: ResMut<Inventory>,
    mut cmd: Commands
) {
    if input.just_pressed(KeyCode::E) {
        if let Ok(trans) = player_query.get_single() {
            for (e, item_trans, item) in item_query.iter() {


                if item_trans.compute_transform().translation.distance(trans.compute_transform().translation) >= 15.0 {
                    continue;
                }
            
                inventory.inventory.push(item.clone());

                cmd.entity(e).despawn();
            }
        }
    }
}

// @Mikal
pub fn check_inv(inv: Res<Inventory>, mut active_effects : ResMut<ActivePlayerEffects>) {
    for i in inv.inventory.iter() {
        match i {
            ItemType::ItemType0 => active_effects.invert = true,
            ItemType::ItemType1 => active_effects.continous_move = true,
            ItemType::ItemType2 => active_effects.rotating_world = true,
            ItemType::ItemType3 => active_effects.moon_gravity = true,
            ItemType::ItemType4 => {},
        }
    }
}


pub fn pick_up_win(
    input: Res<Input<KeyCode>>,
    item_query: Query<(Entity, &GlobalTransform, &ItemType), With<WinItem>>,
    player_query: Query<&GlobalTransform, With<Player>>,
    mut wc: ResMut<WinCon>,
    mut cmd: Commands,
    mut inv: ResMut<Inventory>
) { 
    if input.just_pressed(KeyCode::E) {
        if let Ok(trans) = player_query.get_single() {
            for (e, item_trans, it) in item_query.iter() {


                if item_trans.compute_transform().translation.distance(trans.compute_transform().translation) >= 15.0 {
                    continue;
                }
            
                inv.inventory.push(it.clone());

                cmd.entity(e).despawn();

                wc.0 += 1;
            }
        }
    }
}