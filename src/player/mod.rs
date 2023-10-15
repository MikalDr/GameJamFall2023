use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::platformer::components::{ColliderBundle, GroundDetection};
use crate::status_effects::effects::Inventory;

pub mod systems;

pub mod components;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;


#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_bundle("player.png")]
    pub sprite_bundle: SpriteBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    pub ground_detection: GroundDetection,
    pub inventory: Inventory,

    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,
}


