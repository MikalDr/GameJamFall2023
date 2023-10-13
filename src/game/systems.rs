use bevy::prelude::*;
use crate::game::GameState;

/*
pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>
) 
{
    if keyboard_input.just_pressed(KeyCode::Space) {
        if game_state.0 == Simulation::Running {
            commands.insert_resource(NextState(Some(GameState::Paused)));
            println!("The game is paused");
        }
        if game_state.0 == GameState::Paused {
            commands.insert_resource(NextState(Some(GameState::Running)));
            println!("The game is running");
        }
    }
}*/