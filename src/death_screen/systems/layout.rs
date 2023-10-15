use bevy::prelude::*;

use crate::{main_menu::components::{PlayButton, OptionsButton, MainMenu}, pausemenu::systems::layouts::ReturnButton};

#[derive(Component)]
pub struct DeathMenu{}

#[derive(Component)]
pub struct RetryButton{}

pub fn spawn_death_menu(mut commands: Commands, asset_server: Res<AssetServer>){
    let _death_menu_entity = build_death_menu(&mut commands, asset_server);
}

pub fn despawn_death_menu(mut commands: Commands, death_menu_query: Query<Entity, With<DeathMenu>>){
    if let Ok(death_menu_entity) = death_menu_query.get_single(){
        commands.entity(death_menu_entity).despawn_recursive();
    }
}

pub fn build_death_menu(commands: &mut Commands, asset_server: Res<AssetServer>) -> Entity{
    let death_menu_entity = commands.spawn(
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
            DeathMenu {},
        ))
        .with_children(|parent|{
            // == Title ==
            parent.spawn(
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        height: Val::Px(120.0),
                        width: Val::Px(300.0),
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    /* Image
                    parent.spawn(
                        ImageBundle{
                            ..default()
                        }
                    )*/
                    // == Text ==
                    parent.spawn(
                        TextBundle {
                            text: Text{
                                sections: vec![
                                    TextSection::new("You died.", 
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
                });
            // == Retry ==
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
                RetryButton{},
            )).with_children(|parent|{
                parent.spawn(
                    TextBundle {
                        text: Text{
                            sections: vec![
                                TextSection::new("Retry", 
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
           
            // == Options ==
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
                OptionsButton{},
            )).with_children(|parent|{
                parent.spawn(
                    TextBundle {
                        text: Text{
                            sections: vec![
                                TextSection::new("Options", 
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
            // == Quit ==
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
                    //background_color : Color::BLUE.into(),
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

    death_menu_entity
}