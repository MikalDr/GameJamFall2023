use bevy::{prelude::*, ecs::system::Spawn};
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;

use crate::player::{PlayerBundle, components::ItemBundle, Player};

use self::systems::*;


pub mod components;
pub mod systems;


pub struct PlatformerPlugin;

#[derive(Resource)]
pub struct SpawnLocation {
    pub pos: Vec3
}


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
        .insert_resource(SpawnLocation{
            pos: Vec3{x:0.0, y:0.0, z:0.0}
        })
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
    
        //Spawns something
        .add_systems(Startup, setup)
        .add_systems(Update, process_my_entity)
        .add_systems(Update, spawn_wall_collision)
        .add_systems(Update, spawn_ground_sensor)
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
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_systems(Update, save_spawn_pos);
        
        
    }
}

pub fn spawn_camera(mut commands: Commands){
    let camera = (Camera2dBundle::default(), WorldCamera{});
    commands.spawn(camera);
}

pub fn save_spawn_pos(player_query: Query<&Transform, With<Player>>, mut pos: ResMut<SpawnLocation>) {
    if pos.pos.x == 0.0{
        return;
    }
    if let Ok(player_pos) = player_query.get_single(){
        pos.pos = player_pos.translation;
    }
}