use bevy::prelude::*;

use crate::{AppState, game::GameState};

use self::systems::play_menu_click_sound;

pub mod systems;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, piss);
    }
}


pub fn piss(mut commands: Commands,key_input: Res<Input<KeyCode>>, asset_server: Res<AssetServer>){
    if(key_input.just_pressed(KeyCode::F)){
        play_menu_click_sound(commands, asset_server);
    }
}