use bevy::prelude::*;

use crate::{systems::{toggle_simulation, app_state_call}, pausemenu::PauseMenuPlugin, platformer::systems::is_position_within_level, AppState, player::Player};

pub struct GamePlugin;



impl Plugin for GamePlugin {
    fn build(&self, app: &mut App){
        app.add_state::<GameState>()
        .add_plugins((
            PauseMenuPlugin,
        ))
        .add_systems(Update, kill_player)
        .add_systems(Update, toggle_simulation);
        //.add_systems(Update, app_state_call);
    }
}


//add the state to the plugin later
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    Running,
    #[default]
    Paused,
}


pub fn kill_player(
    mut cmd: Commands,
    camera_query: Query<(&OrthographicProjection, &Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    game_state: Res<State<GameState>>,
    app_state: Res<State<AppState>>
) {
    if game_state.get() == &GameState::Running && app_state.get() == &AppState::Game {
        if let Some(res) = is_position_within_level(camera_query, player_query) {
            if res {
                println!("you dieded");
            }
        }
    }   
}