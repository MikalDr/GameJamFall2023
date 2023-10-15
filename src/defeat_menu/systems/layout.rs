use bevy::prelude::*;

use crate::{pausemenu::systems::layouts::ReturnButton, main_menu::components::QuitButton};

#[derive(Component)]
pub struct DefeatMenu {}

pub fn spawn_defeat_menu(mut commands: Commands, asset_server: Res<AssetServer>){
    let _victory_menu_entity = build_defeat_menu(&mut commands, asset_server);
}

pub fn despawn_defeat_menu(mut commands: Commands, defeat_menu_query: Query<Entity, With<DefeatMenu>>){
    if let Ok(defeat_menu_entity) = defeat_menu_query.get_single(){
        commands.entity(defeat_menu_entity).despawn_recursive();
    }
}

pub fn build_defeat_menu(commands: &mut Commands, asset_server: Res<AssetServer>) -> Entity{
    let defeat_menu_entity = commands.spawn(
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
            DefeatMenu {},
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
                                    TextSection::new("Your time ran out...", 
                                    TextStyle {
                                        font: asset_server.load("upheavtt.ttf"),
                                        font_size: 30.0,
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

            // == Return ==
            /*parent.spawn(
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
               ReturnButton {},
            )).with_children(|parent|{
                parent.spawn(
                    TextBundle {
                        text: Text{
                            sections: vec![
                                TextSection::new("Return", 
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
            });*/
            // == Quit Button ==
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
               QuitButton {},
            )).with_children(|parent|{
                parent.spawn(
                    TextBundle {
                        text: Text{
                            sections: vec![
                                TextSection::new("Quit", 
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

    defeat_menu_entity
}