use std::thread::spawn;

use bevy::prelude::*;

use self::systems::{layout::{spawn_main_menu, despawn_main_menu}, interactions::{interact_with_play_button, interact_with_quit_button, interact_with_credit_button, interact_with_options_button, interact_with_htp_button}};
use crate::AppState;

pub struct MainMenuPlugin;

mod systems;
pub mod components;
mod styles;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
        .add_systems(Update, (
            interact_with_play_button, 
            interact_with_quit_button, 
            interact_with_credit_button,
            interact_with_options_button,
            interact_with_htp_button,
        ));
    }
}

pub fn main_menu() {
}
