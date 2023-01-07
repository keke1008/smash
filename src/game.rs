mod assets;
mod input;
mod objects;
mod tags;

use bevy::prelude::*;

#[derive(Debug)]
pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(assets::GameAssetPlugin)
            .add_plugin(objects::ObjectsPlugin)
            .add_plugin(input::InputPlugin);
    }
}
