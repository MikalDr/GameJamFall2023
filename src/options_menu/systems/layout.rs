use bevy::prelude::*;

#[derive(Component)]
pub struct OptionsMenu {}
#[derive(Component)]
pub struct ResumeButton {}
#[derive(Component)]
pub struct ReturnButton {}

pub fn spawn_options_menu(mut commands: Commands, asset_server: Res<AssetServer>){
    let options_menu_entity = build_options_menu(&mut commands, asset_server);
}

pub fn despawn_options_menu(mut commands: Commands, options_menu_query: Query<Entity, With<OptionsMenu>>){
    if let Ok(options_menu_entity) = options_menu_query.get_single(){
        commands.entity(options_menu_entity).despawn_recursive();
    }
}

pub fn build_options_menu(commands: &mut Commands, asset_server: Res<AssetServer>) -> Entity{
    let options_menu_entity = commands.spawn(
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
                background_color: Color::BLACK.into(),
                ..default()
            },
            OptionsMenu {},
        ))
        .with_children(|parent|{
            // == Text ==
            parent.spawn(
                TextBundle {
                    text: Text{
                        sections: vec![
                            TextSection::new("Paused", 
                            TextStyle {
                                font: asset_server.load("upheavtt.ttf"),
                                font_size: 60.0,
                                color: Color::WHITE.into(),
                            })
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }
            );
            // == Resume Button ==
            parent.spawn(
                (ButtonBundle {
                    style: Style {
                        height: Val::Px(80.0),
                        width: Val::Px(200.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    image: UiImage { 
                        texture: asset_server.load("mainmenu/button.png").into(),
                        ..default()
                    },
                    ..default()
                },
                ResumeButton{},
            )).with_children(|parent|{
                parent.spawn(
                    TextBundle {
                        text: Text{
                            sections: vec![
                                TextSection::new("Resume", 
                                TextStyle {
                                    font: asset_server.load("upheavtt.ttf"),
                                    font_size: 32.0,
                                    ..default()
                                })
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            });
            // == Return Button ==
            parent.spawn(
                (ButtonBundle {
                    style: Style {
                        height: Val::Px(80.0),
                        width: Val::Px(200.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    image: UiImage { 
                        texture: asset_server.load("mainmenu/button.png").into(),
                        ..default()
                    },
                    ..default()
                },
                ReturnButton{},
            )).with_children(|parent|{
                parent.spawn(
                    TextBundle {
                        text: Text{
                            sections: vec![
                                TextSection::new("Main Menu", 
                                TextStyle {
                                    font: asset_server.load("upheavtt.ttf"),
                                    font_size: 32.0,
                                    ..default()
                                })
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            });
        })
        .id();

    options_menu_entity
}