use bevy::prelude::*;

use crate::{AppState, game::GameState};

use self::systems::{layout::{spawn_death_menu, despawn_death_menu}, interactions::interact_with_retry_button};

pub mod systems;

pub struct DeathMenuPlugin;

impl Plugin for DeathMenuPlugin {
    fn build(&self, app: &mut App) {
        app;
        //.add_systems(OnEnter(GameState::Dead), spawn_death_menu)
        //.add_systems(OnExit(GameState::Dead), despawn_death_menu)
        //.add_systems(Update, interact_with_retry_button);
        
    }
}