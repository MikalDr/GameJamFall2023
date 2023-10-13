use bevy::prelude::*;

mod systems;
pub mod player;
pub struct GamePlugin;

/*
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App){
        app.add_state::<GameState>();
    }
}
*/


//add the state to the plugin later
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    Running,
    #[default]
    Paused,
}