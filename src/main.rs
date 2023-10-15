mod utils;
mod systems;
mod platformer;
mod player;
// == Menues ==
mod main_menu;
mod pausemenu;
mod death_screen;
mod credits_menu;
mod victory_menu;
mod defeat_menu;
mod htp_menu;

mod game;
mod sound_controller;
mod timer;

use bevy::prelude::*;
use credits_menu::CreditsMenuPlugin;
use defeat_menu::DefeatMenuPlugin;
use htp_menu::HTPMenuPlugin;
use sound_controller::SoundPlugin;
use systems::*;
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_rapier2d::prelude::*;
use main_menu::{MainMenuPlugin};
use death_screen::DeathMenuPlugin;
use platformer::PlatformerPlugin;
use game::GamePlugin;
use timer::TimerPlugin;
use victory_menu::{systems::layout::VictoryMenu, VictoryMenuPlugin};



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
                VictoryMenuPlugin,
                DefeatMenuPlugin,
                HTPMenuPlugin,
                TimerPlugin,
                SoundPlugin, //MainMenu
                GamePlugin, //GameLogic
            )
        )
        //.add_plugins(WorldInspectorPlugin::new()) // Debug
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
    HowToPlay,
}
