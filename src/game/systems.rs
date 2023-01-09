mod cube_spawner;

use bevy::prelude::*;

#[derive(Debug)]
pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(cube_spawner::CubeSpawnerPlugin);
    }
}
