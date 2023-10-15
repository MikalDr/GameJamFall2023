use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkLevel;

use crate::{AppState, game::{GameState, HasPlayerDied}, player::Player, platformer::{systems::restart_level, SpawnLocation}};

pub fn transistion_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G){
        if app_state.get() == &AppState::Game{
            commands.insert_resource(NextState(Some(AppState::Game)));
        }
    }
}

pub fn transistion_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G){
        if app_state.get() == &AppState::Game{
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
        }
    }
}

pub fn app_state_call(
    game_state: Res<State<GameState>>,
    app_state: Res<State<AppState>>)
    {
}

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
    app_state: Res<State<AppState>>
)
{
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if game_state.get() == &GameState::Running {
            if app_state.get() == &AppState::Game{
                commands.insert_resource(NextState(Some(GameState::Paused)));
            }
        }
        if game_state.get() == &GameState::Paused {
            if app_state.get() == &AppState::Game{
            commands.insert_resource(NextState(Some(GameState::Running)));
            }
        }
    }
}

pub fn toggle_death(
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    app_state: Res<State<AppState>>,
    level_query: Query<Entity, With<Handle<LdtkLevel>>>,
    mut hasDied: ResMut<HasPlayerDied>
)
{
    commands.insert_resource(NextState(Some(GameState::Dead)));
    hasDied.died = true;
}

pub fn move_player_in_death(mut hasDied: ResMut<HasPlayerDied>,mut player_query: Query<&mut Transform, With<Player>>, spawn_loc : Res<SpawnLocation>){
    if let Ok(mut player_entity) = player_query.get_single_mut(){
        if(hasDied.died){
        player_entity.translation = spawn_loc.pos;
        hasDied.died = false;
        }
    }
}