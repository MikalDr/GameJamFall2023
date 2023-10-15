use bevy::prelude::*;

use crate::{AppState, game::GameState};

use self::systems::layout::{spawn_defeat_menu, despawn_defeat_menu};

pub mod systems;

pub struct DefeatMenuPlugin;

impl Plugin for DefeatMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Victory), spawn_defeat_menu)
        .add_systems(OnExit(GameState::Victory), despawn_defeat_menu);
    }
}