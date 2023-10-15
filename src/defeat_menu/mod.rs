use bevy::prelude::*;

use crate::{AppState, game::GameState};

use self::systems::layout::{spawn_defeat_menu, despawn_defeat_menu};

pub mod systems;

pub struct DefeatMenuPlugin;

impl Plugin for DefeatMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Lost), spawn_defeat_menu)
        .add_systems(OnExit(GameState::Lost), despawn_defeat_menu);
    }
}