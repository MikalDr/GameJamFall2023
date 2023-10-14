use bevy::prelude::*;

use super::{components::{Item, Inventory}, Player};

pub fn pick_up_item(
    input: Res<Input<KeyCode>>,
    item_query: Query<(Entity, &Transform, &Item), With<Item>>,
    player_query: Query<&Transform, With<Player>>,
    mut inventory: ResMut<Inventory>,
    mut cmd: Commands
) {
    if input.just_pressed(KeyCode::E) {

        if let Ok(trans) = player_query.get_single() {
            for (e, item_trans, item) in item_query.iter() {
                if item_trans.translation.distance(trans.translation) >= 0.1 {
                    continue;
                }

                inventory.inventory.push(*item);

                cmd.entity(e).despawn();
            }
        }
    }
}