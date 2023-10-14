use bevy::prelude::*;

#[derive(Component)]
pub struct PauseMenu;

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>){
    let pause_menu_entity = build_pause_menu(&mut commands, asset_server);
}

pub fn despawn_pause_menu(mut commands: Commands, pause_menu_query: Query<Entity, With<PauseMenu>>){
    if let Ok(pause_menu_entity) = pause_menu_query.get_single(){
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

pub fn build_pause_menu(commands: &mut Commands, asset_server: Res<AssetServer>) -> Entity{
    let pause_menu_entity = commands.spawn(
        (NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.0), 
                    width: Val::Percent(100.0), 
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::GREEN.into(),
                ..default()
            },
            PauseMenu {},
        ))
        .id();

    pause_menu_entity
}