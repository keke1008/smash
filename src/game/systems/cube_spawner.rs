mod constants;

use std::{marker::PhantomData, time::Duration};

use bevy::{prelude::*, time::Stopwatch};
use iyes_loopless::prelude::*;
use rand::prelude::*;

use crate::{game::objects::SpawnCube, state::AppState};

use self::constants::{
    MetalCube, SpawnableCubes, StoneCube, WoodenCube, NUM_SPAWNABLE_POSITIONS, SPAWN_X_OFFSET,
    SPAWN_Z_POSITION,
};

#[derive(Debug)]
pub(super) struct CubeSpawnerPlugin;

impl Plugin for CubeSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, setup)
            .add_system(update.run_in_state(AppState::InGame))
            .add_exit_system(AppState::InGame, destroy)
            .add_system_set(
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

#[derive(Resource, Debug)]
struct SpawnAmount<Cube> {
    phantom: PhantomData<fn() -> Cube>,
    count: usize,
}

impl<Cube> Default for SpawnAmount<Cube> {
    fn default() -> Self {
        Self {
            phantom: Default::default(),
            count: 0,
        }
    }
}

impl<Cube: SpawnableCubes> SpawnAmount<Cube> {
    fn increment_spawn_count(&mut self) {
        self.count += 1;
    }

    fn get_amount(&self) -> usize {
        (self.count / Cube::INCREASE_INTERVAL_TIMES + 1).min(NUM_SPAWNABLE_POSITIONS)
    }
}

#[derive(Resource, Debug)]
struct SpawnInterval<Cube> {
    phantom: PhantomData<fn() -> Cube>,
    timer: Timer,
}

impl<Cube: SpawnableCubes> Default for SpawnInterval<Cube> {
    fn default() -> Self {
        Self {
            phantom: Default::default(),
            timer: Timer::new(
                Duration::from_secs(Cube::SPAWN_INTERVAL_SEC),
                TimerMode::Repeating,
            ),
        }
    }
}

impl<Cube: SpawnableCubes> SpawnInterval<Cube> {
    fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    fn start_spawning(&self) -> bool {
        self.timer.just_finished()
    }
}

fn spawn_cube<Cube: SpawnableCubes>(
    mut commands: Commands,
    mut interval: Local<SpawnInterval<Cube>>,
    mut amount: Local<SpawnAmount<Cube>>,
    time: Res<Time>,
) {
    interval.tick(time.delta());
    if !interval.start_spawning() {
        return;
    }
    amount.increment_spawn_count();

    let spawn_mask: Vec<_> = (0..NUM_SPAWNABLE_POSITIONS).collect();
    let mut rng = thread_rng();
    for &index in spawn_mask.choose_multiple(&mut rng, amount.get_amount()) {
        let x = index as f32 + SPAWN_X_OFFSET;
        commands.add(SpawnCube {
            position: Vec3::new(x, Cube::SPAWN_Y_POSITION, SPAWN_Z_POSITION),
            cube_type: Cube::TYPE,
        });
    }
}
