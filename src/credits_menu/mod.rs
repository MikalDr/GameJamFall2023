use bevy::prelude::*;

use crate::AppState;

use self::systems::layout::{spawn_credits_menu, despawn_credits_menu};
pub mod systems;

pub struct CreditsMenuPlugin;

impl Plugin for CreditsMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Credits), spawn_credits_menu)
        .add_systems(OnExit(AppState::Credits), despawn_credits_menu);
    }
}