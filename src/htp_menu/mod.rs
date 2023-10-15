use bevy::prelude::*;

use crate::AppState;

use self::systems::layout::{spawn_htp_menu, despawn_htp_menu};

pub mod systems;

pub struct HTPMenuPlugin;

impl Plugin for HTPMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::HowToPlay), spawn_htp_menu)
        .add_systems(OnExit(AppState::HowToPlay), despawn_htp_menu);
    }
}