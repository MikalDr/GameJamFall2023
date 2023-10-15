use bevy::prelude::*;

use crate::pausemenu::systems::layouts::ReturnButton;

#[derive(Component)]
pub struct HTPMenu {}

pub fn spawn_htp_menu(mut commands: Commands, asset_server: Res<AssetServer>){
    println!("Entered HowToPlay menu");
    let _htp_menu_entity = build_htp_menu(&mut commands, asset_server);
}

pub fn despawn_htp_menu(mut commands: Commands, htp_menu_query: Query<Entity, With<HTPMenu>>){
    if let Ok(htp_menu_entity) = htp_menu_query.get_single(){
        commands.entity(htp_menu_entity).despawn_recursive();
    }
}

pub fn build_htp_menu(commands: &mut Commands, asset_server: Res<AssetServer>) -> Entity{
    let credits_menu_entity = commands.spawn(
        (ImageBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.0), 
                    width: Val::Percent(100.0), 
                    height: Val::Percent(100.0),
                    ..default()
                },
                //background_color: Color::BLACK.into(),
                image: asset_server.load("htp.png").into(),
                ..default()
            },
            HTPMenu {},
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
                                    TextSection::new("", 
                                    TextStyle {
                                        font: asset_server.load("upheavtt.ttf"),
                                        font_size: 40.0,
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
                //== TEXTBOX ==
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
                        // == Text ==
                        parent.spawn(
                            TextBundle {
                                text: Text{
                                    sections: vec![
                                        TextSection::new("", 
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
                            // == Text ==
                            parent.spawn(
                                TextBundle {
                                    text: Text{
                                        sections: vec![
                                            TextSection::new("", 
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
                            // == Text ==
                            parent.spawn(
                                TextBundle {
                                    text: Text{
                                        sections: vec![
                                            TextSection::new("", 
                                            TextStyle {
                                                font: asset_server.load("upheavtt.ttf"),
                                                font_size: 40.0,
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
            });
        })
        .id();

    credits_menu_entity
}