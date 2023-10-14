use bevy::prelude::*;

use crate::{AppState, game::GameState};

pub fn transistion_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G){
        if app_state.get() == &AppState::Game{
            commands.insert_resource(NextState(Some(AppState::Game)));
            println!("Game state started");
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
            println!("Main state started");
        }
    }
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
                println!("The game is paused");
            }
        }
        if game_state.get() == &GameState::Paused {
            if app_state.get() == &AppState::Game{
            commands.insert_resource(NextState(Some(GameState::Running)));
            println!("The game is running");
            }
        }
    }
}