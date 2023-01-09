mod constants;

use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use iyes_loopless::prelude::*;
use rand::{prelude::*, Fill};

use crate::{game::objects::SpawnCube, state::AppState};

use self::constants::{
    MetalCube, SpawnableCubes, StoneCube, WoodenCube, FIXED_TIMESTAMP_LABEL,
    NUM_SPAWNABLE_POSITIONS, SPAWN_INTERVAL_SEC, SPAWN_X_OFFSET, SPAWN_Z_POSITION,
};

#[derive(Debug)]
pub(super) struct CubeSpawnerPlugin;

impl Plugin for CubeSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, setup)
            .add_system(update.run_in_state(AppState::InGame))
            .add_exit_system(AppState::InGame, destroy)
            .add_fixed_timestep(
                Duration::from_secs(SPAWN_INTERVAL_SEC),
                FIXED_TIMESTAMP_LABEL,
            )
            .add_fixed_timestep_system_set(
                FIXED_TIMESTAMP_LABEL,
                0,
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(spawn_cube::<WoodenCube>)
                    .with_system(spawn_cube::<StoneCube>)
                    .with_system(spawn_cube::<MetalCube>)
                    .into(),
            );
    }
}

#[derive(Resource, Debug, Default)]
struct GameTime(Stopwatch);

fn setup(mut commands: Commands) {
    commands.init_resource::<GameTime>();
}

fn update(mut game_time: ResMut<GameTime>, time: Res<Time>) {
    game_time.0.tick(time.delta());
}

fn destroy(mut commands: Commands) {
    commands.remove_resource::<GameTime>();
}

fn spawn_cube<Cube: SpawnableCubes>(game_time: Res<GameTime>, mut commands: Commands) {
    if game_time.0.elapsed_secs() <= Cube::TIME_TO_START_SPAWNING as f32 {
        return;
    }

    let mut spawn_mask = vec![false; NUM_SPAWNABLE_POSITIONS];
    let mut rng = thread_rng();
    spawn_mask.try_fill(&mut rng).unwrap();

    for (index, _) in spawn_mask.iter().enumerate().filter(|(_, mask)| **mask) {
        let x = index as f32 + SPAWN_X_OFFSET;
        commands.add(SpawnCube {
            position: Vec3::new(x, Cube::SPAWN_Y_POSITION, SPAWN_Z_POSITION),
            cube_type: Cube::TYPE,
        });
    }
}
