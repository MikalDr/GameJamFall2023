use bevy::prelude::*;

use crate::{systems::toggle_simulation, pausemenu::PauseMenuPlugin};


pub struct GamePlugin;



impl Plugin for GamePlugin {
    fn build(&self, app: &mut App){
        app.add_state::<GameState>()
        .add_plugins((
            PauseMenuPlugin,
        ))
        .add_systems(Update,toggle_simulation);
    }
}


//add the state to the plugin later
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    Running,
    #[default]
    Paused,
}
