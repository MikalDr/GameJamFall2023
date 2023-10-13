use bevy::prelude::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>){
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(){
    
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity{
    let main_menu_entity = commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0), 
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::GREEN.into(),
            ..default()
        })
        .id();

    main_menu_entity
}