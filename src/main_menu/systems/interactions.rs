use bevy::{prelude::*, app::AppExit};

use crate::{main_menu::components::*, AppState, sound_controller::systems::play_menu_click_sound};

pub fn interact_with_play_button(
    mut commands: Commands,
    mut button_query: Query<(&Interaction, &mut UiImage, &mut BackgroundColor), (Changed<Interaction>, With<PlayButton>)>,
    mut app_state_next_state: ResMut<NextState<AppState>> ,
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
                app_state_next_state.set(AppState::Game);
                play_menu_click_sound(commands, asset_server);
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

pub fn interact_with_quit_button(
    mut commands: Commands,
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<(&Interaction, &mut UiImage, &mut BackgroundColor), (Changed<Interaction>, With<QuitButton>)>,
    asset_server: Res<AssetServer>
) {
    let normal_button : Handle<Image> = asset_server.load("mainmenu/button.png").into();
    let hover_button : Handle<Image> = asset_server.load("mainmenu/buttonhover.png").into();
    let pressed_color : Color = Color::rgb(23.0,94.0,254.0);

    if let Ok((interaction, mut image, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                image.texture = normal_button;
                *background_color = Color::BLUE.into();
                app_exit_event_writer.send(AppExit);
                play_menu_click_sound(commands, asset_server);
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

pub fn interact_with_options_button(
    mut commands: Commands,
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<(&Interaction, &mut UiImage, &mut BackgroundColor), (Changed<Interaction>, With<OptionsButton>)>,
    asset_server: Res<AssetServer>
) {
    let normal_button : Handle<Image> = asset_server.load("mainmenu/button.png").into();
    let hover_button : Handle<Image> = asset_server.load("mainmenu/buttonhover.png").into();
    let pressed_color : Color = Color::rgb(23.0,94.0,254.0);

    if let Ok((interaction, mut image, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                image.texture = normal_button;
                *background_color = Color::BLUE.into();
                play_menu_click_sound(commands, asset_server);
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

pub fn interact_with_credit_button(
    mut commands: Commands,
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<(&Interaction, &mut UiImage, &mut BackgroundColor), (Changed<Interaction>, With<CreditsButton>)>,
    asset_server: Res<AssetServer>
) {
    let normal_button : Handle<Image> = asset_server.load("mainmenu/button.png").into();
    let hover_button : Handle<Image> = asset_server.load("mainmenu/buttonhover.png").into();
    let pressed_color : Color = Color::rgb(23.0,94.0,254.0);

    if let Ok((interaction, mut image, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                image.texture = normal_button;
                *background_color = Color::BLUE.into();
                play_menu_click_sound(commands, asset_server);
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