use std::{marker::PhantomData, time::Duration};

use bevy::prelude::*;
use rand::prelude::SliceRandom;

use crate::game::objects::SpawnCube;

use super::constants::{SpawnableCubes, NUM_SPAWNABLE_POSITIONS, SPAWN_X_OFFSET, SPAWN_Z_POSITION};

#[derive(Resource, Debug)]
pub(super) struct SpawnControl<Cube: SpawnableCubes> {
    phantom: PhantomData<fn() -> Cube>,
    spawn_times: usize,
    timer: Timer,
}

impl<Cube: SpawnableCubes> Default for SpawnControl<Cube> {
    fn default() -> Self {
        Self {
            phantom: Default::default(),
            spawn_times: 0,
            timer: Timer::new(
                Duration::from_secs(Cube::SPAWN_INTERVAL_SEC),
                TimerMode::Repeating,
            ),
        }
    }
}

impl<Cube: SpawnableCubes> SpawnControl<Cube> {
    pub(super) fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    pub(super) fn get_spawn_amount(&mut self) -> Option<usize> {
        if self.timer.just_finished() {
            self.spawn_times += 1;
            let unlimited_times = self.spawn_times / Cube::INCREASE_INTERVAL_TIMES + 1;
            Some(unlimited_times.min(NUM_SPAWNABLE_POSITIONS))
        } else {
            None
        }
    }
}

pub(super) fn generate_spawn_index<Cube: SpawnableCubes>(amount: usize) -> Vec<usize> {
    let mut spawn_mask = [0; NUM_SPAWNABLE_POSITIONS];
    for (index, dest) in spawn_mask.iter_mut().enumerate() {
        *dest = index;
    }

    let mut rng = rand::thread_rng();
    spawn_mask
        .choose_multiple(&mut rng, amount)
        .copied()
        .collect::<Vec<_>>()
}

pub(super) fn generate_spawn_commands<Cube: SpawnableCubes>(
    spawn_index: &[usize],
) -> impl Iterator<Item = SpawnCube> + '_ {
    spawn_index.iter().map(|&index| {
        let x = index as f32 + SPAWN_X_OFFSET;
        SpawnCube {
            cube_type: Cube::TYPE,
            position: Vec3::new(x, Cube::SPAWN_Y_POSITION, SPAWN_Z_POSITION),
        }
    })
}
