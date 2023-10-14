use bevy::prelude::*;
use super::GameState;

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>
) 
{
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if game_state.get() == &GameState::Running {
            commands.insert_resource(NextState(Some(GameState::Paused)));
            println!("The game is paused");
        }
        if game_state.get() == &GameState::Paused {
            commands.insert_resource(NextState(Some(GameState::Running)));
            println!("The game is running");
        }
    }
}