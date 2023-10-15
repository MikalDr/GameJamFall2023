use bevy::prelude::*;

use crate::platformer::components::{Item, WinItem, WinCon};

use super::{components::Inventory, Player};

pub fn pick_up_item(
    input: Res<Input<KeyCode>>,
    item_query: Query<(Entity, &GlobalTransform, &Item)>,
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



pub fn pick_up_win(
    input: Res<Input<KeyCode>>,
    item_query: Query<(Entity, &GlobalTransform), With<WinItem>>,
    player_query: Query<&GlobalTransform, With<Player>>,
    mut wc: ResMut<WinCon>,
    mut cmd: Commands
) {
    if input.just_pressed(KeyCode::E) {
        if let Ok(trans) = player_query.get_single() {
            for (e, item_trans) in item_query.iter() {


                if item_trans.compute_transform().translation.distance(trans.compute_transform().translation) >= 15.0 {
                    continue;
                }
            


                cmd.entity(e).despawn();

                wc.0 += 1;
            }
        }
    }
}