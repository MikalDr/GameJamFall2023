use bevy::prelude::*;

mod systems;
pub struct GamePlugin;
pub mod level_controller;
pub mod platformer;

use self::{pausemenu::PauseMenuPlugin, level_controller::LevelControllerPlugin, systems::toggle_simulation};

pub mod pausemenu;


impl Plugin for GamePlugin {
    fn build(&self, app: &mut App){
        app.add_state::<GameState>()
        .add_plugins((
            PauseMenuPlugin,
            LevelControllerPlugin,
            PlayerPlugin,
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
