mod camera;
mod cube;
mod light;
mod player;
mod punch;
mod stage;

use bevy::prelude::*;

pub(in crate::game) use self::cube::{CubeAssets, CubeType, SpawnCube};
pub(in crate::game) use self::player::PlayerAssets;

#[derive(Debug)]
pub(super) struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(stage::StagePlugin)
            .add_plugin(light::LightPlugin)
            .add_plugin(camera::CameraPlungin)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(punch::PunchPlugin)
            .add_plugin(cube::CubePlugin);
    }
}
