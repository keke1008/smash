mod assets;
mod constants;
mod groups;
mod input;
mod objects;
mod systems;
mod tags;

use bevy::prelude::*;

#[derive(Debug)]
pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(assets::GameAssetPlugin)
            .add_plugin(objects::ObjectsPlugin)
            .add_plugin(input::InputPlugin)
            .add_plugin(systems::SystemsPlugin);
    }
}
