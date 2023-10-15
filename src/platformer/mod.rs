use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;

use crate::player::{PlayerBundle, systems::pick_up_item, components::Inventory};

use self::{systems::*, components::ItemBundle};


pub mod components;
pub mod systems;


pub struct PlatformerPlugin;


impl Plugin for PlatformerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
        .add_plugins((
            LdtkPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -2000.0),
            ..Default::default()
        })
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .insert_resource(Inventory { inventory: Vec::new() })
        //Spawns something
        .add_systems(Startup, setup)
        .add_systems(Update, process_my_entity)
        .add_systems(Update, spawn_wall_collision)
        .add_systems(Update, spawn_ground_sensor)
        .add_systems(Update, pick_up_item)
        //.add_systems(Startup, apply_player_sprite)
        //.add_systems(Update, player_debug)
        .add_systems(Update, animate_sprite)
        .add_systems(Update, movement)
        //.add_systems(Update, camera_fit_inside_current_level)
        //.add_systems(Update, update_level_selection)
        .add_systems(Update, ground_detection)
        .add_systems(Update, update_on_ground)
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_int_cell::<components::WallBundle>(3)
        .register_ldtk_entity::<ItemBundle>("Item")
        .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

pub fn spawn_camera(mut commands: Commands){
    let camera = (Camera2dBundle::default(), WorldCamera{});
    commands.spawn(camera);
}