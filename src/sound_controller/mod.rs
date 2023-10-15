use bevy::{prelude::*, audio::PlaybackMode};

use crate::{AppState, game::GameState};

use self::systems::play_menu_click_sound;

pub mod systems;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, piss)
            .add_systems(Startup, main_menu_music)
            .add_systems(OnEnter(AppState::Game), despawn_main_menu_music)
            .add_systems(OnEnter(AppState::Game), game_music)
            .add_systems(OnExit(AppState::Game), game_music_pause);
    }
}


pub fn piss(mut commands: Commands,key_input: Res<Input<KeyCode>>, asset_server: Res<AssetServer>){
    if key_input.just_pressed(KeyCode::F) {
        play_menu_click_sound(&mut commands, asset_server);
    }
}


pub fn main_menu_music(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn((AudioBundle {
        source: asset_server.load("sounds/main_menu.ogg"),
        settings: PlaybackSettings { 
            mode: PlaybackMode::Loop,
            ..default()
        }
    },
    MainMenuMusic
));
}

pub fn despawn_main_menu_music(mut cmd: Commands, query: Query<Entity, With<MainMenuMusic>>) {
    if let Ok(e) = query.get_single() {
        cmd.entity(e).despawn();
    }
}

#[derive(Component)]
pub struct GameMusic;

#[derive(Component)]
pub struct MainMenuMusic;

pub fn game_music(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn((AudioBundle {
        source: asset_server.load("sounds/game_music.ogg"),
        settings: PlaybackSettings { 
            mode: PlaybackMode::Loop,
            ..default()
        }
    },
    GameMusic
    ));
}

pub fn game_music_pause(mut cmd: Commands, mut query: Query<Entity, With<GameMusic>>) {
    if let Ok(e) = query.get_single_mut() {
        cmd.entity(e).despawn();
    }
}


