use bevy::prelude::*;

use crate::{AppState, game::GameState};

use self::systems::layout::{spawn_victory_menu, despawn_victory_menu};

pub mod systems;

pub struct VictoryMenuPlugin;

impl Plugin for VictoryMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Victory), spawn_victory_menu)
        .add_systems(OnExit(GameState::Victory), despawn_victory_menu);
    }
}