use bevy::prelude::*;

use crate::{pausemenu::systems::layouts::ReturnButton, main_menu::components::QuitButton};

#[derive(Component)]
pub struct TimerHud {}

pub fn spawn_timer(mut commands: Commands, asset_server: Res<AssetServer>){
    println!("Timer spawned");
    let _victory_menu_entity = build_timer(&mut commands, asset_server);
}

pub fn despawn_timer(mut commands: Commands, timer_query: Query<Entity, With<TimerHud>>){
    if let Ok(timer_entity) = timer_query.get_single(){
        commands.entity(timer_entity).despawn_recursive();
    }
}

pub fn build_timer(commands: &mut Commands, asset_server: Res<AssetServer>) -> Entity{
    let timer_entity = commands.spawn(
        (NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Default,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.0), 
                    width: Val::Percent(100.0), 
                    height: Val::Percent(50.0),
                    ..default()
                },
                background_color: bevy::prelude::BackgroundColor(Color::Rgba{
                        /// Red channel. [0.0, 1.0]
                        red: 0.0,
                        /// Green channel. [0.0, 1.0]
                        green: 0.0,
                        /// Blue channel. [0.0, 1.0]
                        blue: 0.0,
                        /// Alpha channel. [0.0, 1.0]
                        alpha: 0.0,
                }),
                ..default()
            },
            TimerHud {},
        ))
        .with_children(|parent|{
            // == Title ==
            parent.spawn(
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        height: Val::Px(50.0),
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
                                    TextSection::new("20:00", 
                                    TextStyle {
                                        font: asset_server.load("upheavtt.ttf"),
                                        font_size: 45.0,
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
        })
        .id();

        timer_entity
}