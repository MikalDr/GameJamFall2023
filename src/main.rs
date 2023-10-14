mod utils;
mod systems;
mod platformer;
// == Menues ==
mod main_menu;
mod pausemenu;
mod options_menu;
mod death_screen;
mod credits_menu;

mod game;
mod sound_controller;

use bevy::prelude::*;
use credits_menu::CreditsMenuPlugin;
use sound_controller::SoundPlugin;
use systems::*;
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_rapier2d::prelude::*;
use main_menu::{MainMenuPlugin};
use death_screen::DeathMenuPlugin;
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
                //MainMenuPlugin, //MainMenu
                //GamePlugin, //GameLogic
                PlatformerPlugin,
                MainMenuPlugin,
                DeathMenuPlugin,
                CreditsMenuPlugin,
                SoundPlugin, //MainMenu
                GamePlugin, //GameLogic
            )
        )
        .run();
}


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Credits,
    Options,
    Game,
    GameOver,
}
