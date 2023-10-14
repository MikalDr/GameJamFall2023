use bevy::prelude::*;

use crate::AppState;

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