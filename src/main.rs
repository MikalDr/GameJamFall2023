use bevy::prelude::*;
use systems::*;
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_rapier2d::prelude::*;
use game::level_controller::LevelControllerPlugin;

mod game;
mod main_menu;
mod utils;



fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,
                LdtkPlugin,
                LevelControllerPlugin
            )
        )
        .add_state::<AppState>()
        .add_systems(Startup, setup_graphics)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
