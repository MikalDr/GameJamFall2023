/*pub fn spawn_main_menu_music(mut commands: Commands, asset_server: Res<AssetServer>){
    let main_menu_music = commands.spawn(AudioBundle {
        source: asset_server.load("sounds/Windless Slopes.ogg"),
        ..default()
    });
}*/

use bevy::{prelude::*, audio::PlaybackMode};

pub fn play_menu_click_sound(commands: &mut Commands, asset_server: Res<AssetServer>){
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/menu_click.ogg"),
        settings: PlaybackSettings { 
            mode: PlaybackMode::Once,
            ..default()
         }
    });
}

/*pub fn despawn_main_menu_music(mut commands: Commands, pause_menu_query: Query<Entity, With<PauseMenu>>){
    if let Ok(pause_menu_entity) = pause_menu_query.get_single(){
        commands.entity(pause_menu_entity).despawn_recursive();
    }
} */