use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkLevel;

use crate::{game::GameState, AppState, sound_controller::systems::play_menu_click_sound, platformer::systems::restart_level};

use super::layout::RetryButton;

pub fn interact_with_retry_button(
    mut commands: Commands,
    mut button_query: Query<(&Interaction, &mut UiImage, &mut BackgroundColor), (Changed<Interaction>, With<RetryButton>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    level_query: Query<Entity, With<Handle<LdtkLevel>>>,
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
                app_state_next_state.set(AppState::Game);
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

pub fn quick_respawn(    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut game_state_next_state: ResMut<NextState<GameState>>) {
    game_state_next_state.set(GameState::Running);
    app_state_next_state.set(AppState::Game);
}