use bevy::prelude::*;
use systems::*;
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_rapier2d::prelude::*;

mod main_menu;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,
                LdtkPlugin,
            )
        .add_state::<AppState>()
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
