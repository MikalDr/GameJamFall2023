use bevy::prelude::*;

use self::systems::layouts::{despawn_pause_menu, spawn_pause_menu};

use super::GameState;

mod systems;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
        .add_systems(OnExit(GameState::Paused), despawn_pause_menu);
        //.add_systems(Update, (interact_with_play_button, interact_with_quit_button));
    }
}

pub fn pause_menu() {
    println!("Pause menu started.");
}