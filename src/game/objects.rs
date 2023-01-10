mod camera;
mod cube;
mod light;
mod player;
mod punch;
mod stage;
mod ui;

use bevy::prelude::*;

pub(in crate::game) use self::cube::{CubeAssets, CubeType, SpawnCube};
pub(in crate::game) use self::player::PlayerAssets;
pub(in crate::game) use self::ui::UiAssets;

#[derive(Debug)]
pub(super) struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(stage::StagePlugin)
            .add_plugin(light::LightPlugin)
            .add_plugin(camera::CameraPlungin)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(punch::PunchPlugin)
            .add_plugin(cube::CubePlugin)
            .add_plugin(ui::GameUiPlugin);
    }
}
