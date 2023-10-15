use bevy::prelude::*;

use crate::{AppState, game::GameState};

use self::systems::layout::{spawn_timer, despawn_timer};


pub mod systems;

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app
        //.add_systems(OnEnter(AppState::Game), spawn_timer)
        .add_systems(OnEnter(AppState::Game), spawn_timer)
        .add_systems(OnExit(GameState::Paused), despawn_timer);
    }
}