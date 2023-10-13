use bevy::prelude::*;

use crate::main_menu::components::*;

pub fn interact_with_play_button(
    mut button_query: Query<(&Interaction, &mut UiImage), (Changed<Interaction>, With<PlayButton>)>, asset_server: Res<AssetServer>
) {
    if let Ok((interaction, mut image)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => {
                image.texture = asset_server.load("mainmenu/buttonhover.png");
            }
            Interaction::None => {
                image.texture = asset_server.load("mainmenu/button.png");
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<(&Interaction, &mut UiImage), (Changed<Interaction>, With<QuitButton>)>, asset_server: Res<AssetServer>
) {
    if let Ok((interaction, mut image)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => {
                image.texture = asset_server.load("mainmenu/buttonhover.png");
            }
            Interaction::None => {
                image.texture = asset_server.load("mainmenu/button.png");
            }
        }
    }
}