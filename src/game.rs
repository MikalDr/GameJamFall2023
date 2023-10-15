use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkLevel;

use crate::platformer::SpawnLocation;
use crate::platformer::systems::camera_follow;
use crate::systems::move_player_in_death;
use crate::{systems::toggle_simulation, pausemenu::PauseMenuPlugin, platformer::systems::is_position_within_level, AppState, player::Player};
use crate::{systems::toggle_death, platformer::systems::WorldCamera};


pub struct GamePlugin;

#[derive(Resource)]
pub struct HasPlayerDied {
    pub died: bool
}



impl Plugin for GamePlugin {
    fn build(&self, app: &mut App){
        app.add_state::<GameState>()
        .add_plugins((
            PauseMenuPlugin,
        ))
        .insert_resource(HasPlayerDied{died: false})
        .add_systems(Update, kill_conditions_player)
        .add_systems(Update, toggle_simulation)
        .add_systems(Update, move_player_in_death);
        //.add_systems(Update, app_state_call);
    }
}


//add the state to the plugin later
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    Running,
    #[default]
    Paused,
    Victory,
    Dead,
}


pub fn kill_conditions_player(
    mut cmd: Commands,
    mut camera_query: Query<(&Camera, &GlobalTransform), With<WorldCamera>>,
    player_query: Query<&Transform, With<Player>>,
    game_state: Res<State<GameState>>,
    app_state: Res<State<AppState>>,
    level_query: Query<Entity, With<Handle<LdtkLevel>>>,
    has_died : ResMut<HasPlayerDied>
) {
    println!("{:?} {:?}", game_state.get(), app_state.get());
    if game_state.get() == &GameState::Running && app_state.get() == &AppState::Game {
        
        // Kills player if outside screen
        if let Some(res) = is_position_within_level(camera_query, player_query) {
            if res {
                toggle_death(cmd, game_state, app_state, level_query, has_died);
                println!("you dieded");
            }
        }
    }   
}