use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::AppState;

#[derive(Component)]
pub struct Player;


pub const PLAYER_SPEED: f32 = 10.;

pub fn move_player
(
    mut player_query: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    if let Ok(mut plr) = player_query.get_single_mut() {

        if input.pressed(KeyCode::W) {
            plr.translation.y += PLAYER_SPEED * time.delta_seconds();
        }

        if input.pressed(KeyCode::S) {
            plr.translation.y -= PLAYER_SPEED * time.delta_seconds();
        }
        
        if input.pressed(KeyCode::D) {
            plr.translation.x += PLAYER_SPEED * time.delta_seconds();
        }

        if input.pressed(KeyCode::A) {
            plr.translation.x -= PLAYER_SPEED * time.delta_seconds();
        }
    }
}

pub struct PlayerPlugin;


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), setup)
            .add_systems(Update, animate_sprite);
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn setup
(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {

    let texture_handle = asset_server.load("blacknwhite.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(12.0, 12.0), 4, 1, None, Some(Vec2::new(36., 0.)));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let animation_indices = AnimationIndices { first: 0, last: 3 };

    cmd.spawn(
        (
            Player,
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform::from_scale(Vec3::splat(6.0)),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(12.0),
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        )
    );
}


#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);