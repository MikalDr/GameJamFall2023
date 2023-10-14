use bevy::prelude::*;
use bevy_ecs_ldtk::EntityIid;

use crate::player::Player;

#[derive(Component, Clone, Default, Debug)]
//TODO: Need to put this on something
pub struct PlayerEffects {
    // An array with maximum of 10 effects
    pub effects: [StatusEffect; 10],
}
#[derive(Clone, Default, PartialEq, Debug)]
pub enum StatusEffect {
    #[default]
    None,
    Effect1,
    Effect2,
    Effect3
}

pub fn remove_effect() {}

pub fn add_effect(
    mut commands: Commands,
    key_input: Res<Input<KeyCode>>,
    mut effect_query: Query<&mut PlayerEffects, With<Player>>
) 
{
    if let Ok(player_effects) = effect_query.get_single() {
        
        println!("{:?}", player_effects.effects);
        if(key_input.just_pressed(KeyCode::T)){
            for mut effect in player_effects.effects.iter_mut() {
                if effect == &StatusEffect::None{
                    effect = StatusEffect::Effect1;
                }
            }

        }
        if(key_input.just_pressed(KeyCode::Y)){
            for mut effect in player_effects.effects.iter() {
                println!("HELLO {:?}", effect);
                if effect == &StatusEffect::None{
                    println!("Greetings {:?}", effect);
                    effect = &StatusEffect::Effect2;
                }
            }

        }
        if(key_input.just_pressed(KeyCode::U)){
            for mut effect in player_effects.effects.iter() {
                if effect == &StatusEffect::None{
                    effect = &StatusEffect::Effect3;
                }
            }

        }
    }
}