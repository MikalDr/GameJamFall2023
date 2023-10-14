use bevy::prelude::*;

use self::{pausemenu::PauseMenuPlugin, level_controller::LevelControllerPlugin, player::PlayerPlugin, systems::toggle_simulation};

pub mod systems;
pub mod player;
pub struct GamePlugin;
pub mod level_controller;
pub mod level_systems;
pub mod level_components;
pub mod pausemenu;


impl Plugin for GamePlugin {
    fn build(&self, app: &mut App){
        app.add_state::<GameState>()
        .add_plugins((
            PauseMenuPlugin,
            //LevelControllerPlugin,
            //PlayerPlugin,
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
