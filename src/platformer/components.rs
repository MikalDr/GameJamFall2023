use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::ldtk_pixel_coords_to_translation_pivoted};

use std::{collections::HashSet, str::FromStr};
use thiserror::Error;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(6., 6.),
                rigid_body: RigidBody::Dynamic,
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints,
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

impl From<IntGridCell> for SensorBundle {
    fn from(int_grid_cell: IntGridCell) -> SensorBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        // ladder
        if int_grid_cell.value == 2 {
            SensorBundle {
                collider: Collider::cuboid(6., 6.),
                sensor: Sensor,
                rotation_constraints,
                active_events: ActiveEvents::COLLISION_EVENTS,
            }
        } else {
            SensorBundle::default()
        }
    }
}



#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Clone, Default, Component)]
pub struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}


#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer {
    pub timer: Timer,
}

#[derive(Component, Clone)]
pub struct Item(ItemType);

impl Default for Item {
    fn default() -> Self {
        Self(ItemType::ItemType0)
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ItemBundle {
    pub item: Item,
    #[sprite_bundle("cup.png")]
    pub sprite_bundle: SpriteBundle,
}



#[derive(Reflect, Clone)]
pub enum ItemType {
    ItemType0,
    ItemType1,
    ItemType2,
    ItemType3,
    ItemType4,
}

#[derive(Debug, Error)]
#[error("the given equipment value doesn't exist")]
pub struct NoItemType;


impl FromStr for ItemType {
    type Err = NoItemType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ItemType::*;

        match s {
            "ItemType0" => Ok(ItemType0),
            "ItemType1" => Ok(ItemType1),
            "ItemType2" => Ok(ItemType2),
            "ItemType3" => Ok(ItemType3),
            "ItemType4" => Ok(ItemType4),
            _ => Err(NoItemType),
        }
    }
}

#[derive(Resource)]
pub struct HasWon(pub bool);


#[derive(Resource)]
pub struct WinCon(pub usize, pub usize);


pub fn check_win_con(wc: Res<WinCon>, mut win: ResMut<HasWon>) {
    win.0 =  wc.0 <= wc.1;
}