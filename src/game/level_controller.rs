use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use rand::Rng;
use crate::utils::Const::LEVEL_IDS;

pub struct LevelControllerPlugin;

#[derive(Resource)]
pub struct CurrentLevel {
    pub level: usize
}

impl Default for CurrentLevel {
    fn default() -> Self {
        Self { level: 0 }
    }
}

impl Plugin for LevelControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, toggle_levels)
            .init_resource::<CurrentLevel>()
            .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation  {
                load_level_neighbors: false,
            },
            ..Default::default()
        });
    }
}


fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let level_set = LevelSet::from_iid((*LEVEL_IDS.get(0).unwrap()).to_string());

    cmd.spawn(
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("map\\gjf.ldtk"),
            level_set,
            transform: Transform::from_xyz(-256., -144., 0.),
            ..Default::default()
        }
    );
}


fn toggle_levels(input: Res<Input<KeyCode>>, mut level_sets: Query<&mut LevelSet>, mut lvl: ResMut<CurrentLevel>) {
    if input.just_pressed(KeyCode::Space) {

        if lvl.level >= LEVEL_IDS.len() - 1 {
            return;
        } else {
            lvl.level += 1;
        }

        let level_to_toggle = *LEVEL_IDS.get(lvl.level).unwrap();

        let mut level_set = level_sets.single_mut();
        if level_set.iids.contains(level_to_toggle) {
            level_set.iids.remove(level_to_toggle);
        } else {
            level_set.iids.insert(level_to_toggle.to_string());
        }
    }
}


fn load_level(mut level_sets: Query<&mut LevelSet>, lvl_code: &str) {

}