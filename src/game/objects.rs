mod camera;
mod cube;
mod light;
mod player;
mod stage;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::state::AppState;

pub(in crate::game) use self::cube::CubeAssets;
pub(in crate::game) use self::player::PlayerAssets;

#[derive(Debug)]
pub(super) struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, setup)
            .add_plugin(camera::CameraPlungin)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(cube::CubePlugin);
    }
}

fn setup(mut commands: Commands) {
    commands.add(stage::SpawnStage);
    commands.add(camera::SpawnCamera);
    commands.add(light::SpawnLight);
    commands.add(player::SpawnPlayer);
}
