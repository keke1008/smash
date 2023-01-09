use crate::game::constants::{CUBE_WIDTH, STAGE_DEPTH, STAGE_WIDTH};
use crate::game::objects::CubeType;

pub(super) const NUM_SPAWNABLE_POSITIONS: usize = STAGE_WIDTH as usize / CUBE_WIDTH as usize;
pub(super) const SPAWN_X_OFFSET: f32 = -STAGE_WIDTH / 2.0 + 0.5;
pub(super) const SPAWN_Z_POSITION: f32 = -(STAGE_DEPTH / 2.0) + 1.0;
const BASE_SPAWN_Y_POSITION: f32 = CUBE_WIDTH / 2.0 + 1.0;

pub(super) const FIXED_TIMESTAMP_LABEL: &str = "CubeSpawnTimestep";
pub(super) const SPAWN_INTERVAL_SEC: u64 = 5;

pub(super) trait SpawnableCubes {
    const TYPE: CubeType;
    const SPAWN_Y_POSITION: f32;
    const TIME_TO_START_SPAWNING: u32;
}

#[derive(Debug)]
pub(super) struct WoodenCube;

impl SpawnableCubes for WoodenCube {
    const TYPE: CubeType = CubeType::Wooden;
    const SPAWN_Y_POSITION: f32 = BASE_SPAWN_Y_POSITION + 2.0;
    const TIME_TO_START_SPAWNING: u32 = 0;
}

#[derive(Debug)]
pub(super) struct StoneCube;

impl SpawnableCubes for StoneCube {
    const TYPE: CubeType = CubeType::Stone;
    const SPAWN_Y_POSITION: f32 = BASE_SPAWN_Y_POSITION + 1.0;
    const TIME_TO_START_SPAWNING: u32 = 10;
}

#[derive(Debug)]
pub(super) struct MetalCube;

impl SpawnableCubes for MetalCube {
    const TYPE: CubeType = CubeType::Metal;
    const SPAWN_Y_POSITION: f32 = BASE_SPAWN_Y_POSITION;
    const TIME_TO_START_SPAWNING: u32 = 20;
}
