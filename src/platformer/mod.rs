use bevy::{prelude::*, ecs::system::Spawn};
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;


use crate::{player::{PlayerBundle, systems::{pick_up_item, pick_up_win, check_inv}, components::{Inventory, JumpScareTime, JumpScareEventTimer}, Player}, game::GameState, AppState};

use self::{systems::*, components::{ItemBundle, check_win_con, HasWon, WinCon, WinItemBundle, has_won}};


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
        .insert_resource(HasWon(false)) // Wheter we have won or not
        .insert_resource(WinCon(0, 7))  // How many win-con collectables there are.
        .insert_resource(Inventory { inventory: Vec::new() })
        //Spawns something
        .add_systems(Startup, setup)
        .add_systems(Update, process_my_entity)
        .add_systems(Update, spawn_wall_collision)
        .add_systems(Update, check_inv)
        .add_systems(Update, spawn_ground_sensor)
        .add_systems(Update, pick_up_item)
        //.add_systems(Startup, apply_player_sprite)
        //.add_systems(Update, player_debug)
        .add_systems(Update, animate_sprite)
        .insert_resource(Controls{
            right: KeyCode::D,
            left: KeyCode::A,
            jump: KeyCode::Space,
            non_stop_move: false,
            rx: 0.,
            lx: 0.,
        })
        .insert_resource(HasLost(false))
        .insert_resource(Scare(false))
        .insert_resource(JumpScareEventTimer(Timer::from_seconds(3.0, TimerMode::Once)))
        .add_systems(Update, has_lost)
        .add_systems(Update, movement)
        .add_systems(Update, jump_scare)
        //.add_systems(Update, camera_fit_inside_current_level)
        //.add_systems(Update, update_level_selection)
        .add_systems(Update, ground_detection)
        .add_systems(Update, update_on_ground)
        .insert_resource(JumpScareTime(Timer::from_seconds(3., TimerMode::Once)))
        .add_systems(Update, save_spawn_pos)
        .add_systems(Update, check_win_con)
        .add_systems(Update, pick_up_win)
        .add_systems(Update, has_won)
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_int_cell::<components::WallBundle>(3)
        .register_ldtk_entity::<ItemBundle>("Item")
        .register_ldtk_entity::<WinItemBundle>("WinCon")
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_systems(Update, camera_follow)
        .add_systems(Update, timer.run_if(in_state(GameState::Running)))
        .add_systems(Update, timer_1)
        .add_systems(Update, tick_timer.run_if(in_state(GameState::Running).and_then(in_state(AppState::Game))))
        .add_systems(Update, tick_event_timer)
        .add_systems(Update, scare)
        .add_systems(Update, invert_controls)
        .add_systems(Update, non_stop_movement);
        
        
    }
}

pub fn spawn_camera(mut commands: Commands){
    let camera = (Camera2dBundle
        {
        transform: Transform{
            scale: Vec3{x:0.5,y:0.5,z:0.5},
            ..default()
        },
        ..default()
        }
        ,WorldCamera{});
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

#[derive(Resource)]
pub struct Scare(pub bool);

pub fn tick_timer(mut t: ResMut<JumpScareTime>, time: Res<Time>) {
    t.0.tick(time.delta());
}

pub fn tick_event_timer(mut t: ResMut<JumpScareEventTimer>, time: Res<Time>, l: Res<HasLost>, mut s: ResMut<Scare>) {
    if l.0 {
        t.0.tick(time.delta());

        if t.0.finished() {
            s.0 = true;
        }
    }
}


#[derive(Component)]
pub struct JumpScareSound;

pub fn scare(mut cmd: Commands, asset_server: Res<AssetServer>, s: Res<Scare>) {
    if s.0 {
        cmd.spawn(
            (
                AudioBundle {
                    source: asset_server.load("sounds/jumpscare.ogg"),
                    ..Default::default()
                },
                JumpScareSound
            )
        );

        cmd.spawn(
            SpriteBundle {
                texture: asset_server.load("shoes.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 500., y: 500. }),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 10.),
                ..default()
        });
    }
}