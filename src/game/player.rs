use bevy::prelude::*;

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