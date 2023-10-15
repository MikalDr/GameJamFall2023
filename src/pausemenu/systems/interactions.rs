use bevy::{prelude::*, app::AppExit};
use bevy_ecs_ldtk::LdtkLevel;

use crate::{game::{GameState, HasPlayerDied}, AppState, sound_controller::systems::play_menu_click_sound, platformer::systems::restart_level};

use super::layouts::{ResumeButton, ReturnButton};

pub fn interact_with_resume_button(
    mut commands: Commands,
    mut button_query: Query<(&Interaction, &mut UiImage, &mut BackgroundColor), (Changed<Interaction>, With<ResumeButton>)>,
     mut game_state_next_state: ResMut<NextState<GameState>>,
     asset_server: Res<AssetServer>
) {
    let normal_button : Handle<Image> = asset_server.load("mainmenu/button.png").into();
    let hover_button : Handle<Image> = asset_server.load("mainmenu/buttonhover.png").into();
    //does not work
    //let pressed_color : Color = Color::rgb(23.0,94.0,254.0);

    if let Ok((interaction, mut image, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                image.texture = normal_button;
                *background_color = Color::BLUE.into();
                play_menu_click_sound(&mut commands, asset_server);
                game_state_next_state.set(GameState::Running);
            }
            Interaction::Hovered => {
                image.texture = asset_server.load("mainmenu/buttonhover.png").into();
            }
            Interaction::None => {
                image.texture = asset_server.load("mainmenu/button.png").into();
            }
        }
    }
}

pub fn interact_with_return_button(
    mut commands: Commands,
    mut button_query: Query<(&Interaction, &mut UiImage, &mut BackgroundColor), (Changed<Interaction>, With<ReturnButton>)>, asset_server: Res<AssetServer>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut has_died: ResMut<HasPlayerDied>,
) {
    let normal_button : Handle<Image> = asset_server.load("mainmenu/button.png").into();
    let hover_button : Handle<Image> = asset_server.load("mainmenu/buttonhover.png").into();
    let pressed_color : Color = Color::rgb(23.0,94.0,254.0);

    if let Ok((interaction, mut image, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                image.texture = normal_button;
                *background_color = Color::BLUE.into();
                play_menu_click_sound(&mut commands, asset_server);
                has_died.died = true;
                game_state_next_state.set(GameState::Paused);
                app_state_next_state.set(AppState::MainMenu);
            }
            Interaction::Hovered => {
                image.texture = hover_button;
                *background_color = Color::WHITE.into();
            }
            Interaction::None => {
                image.texture = normal_button;
                *background_color = Color::WHITE.into();
            }
        }
    }
}