use bevy::prelude::*;

use self::effects::add_effect;

pub mod effects;
#[derive(Component)]
pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Update, add_effect);
    }
}
