mod main_menu;
mod utils;
mod systems;
mod platformer;
mod pausemenu;
mod game;

use bevy::prelude::*;
use systems::*;
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_rapier2d::prelude::*;
use main_menu::MainMenuPlugin;
use platformer::PlatformerPlugin;
use game::GamePlugin;



fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Snils Og Knalle".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }) // Window stuff
            .set(ImagePlugin::default_nearest()) // Prevents blurry pics?
        )
        .add_state::<AppState>()
        .add_plugins(
            (
                PlatformerPlugin,
                MainMenuPlugin, //MainMenu
                GamePlugin, //GameLogic
            )
        )
        /*
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        */
        .add_systems(Startup, setup_graphics)
        .run();
}

fn setup_graphics(mut cmd: Commands) {
    //cmd.spawn(Camera2dBundle::default());
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
