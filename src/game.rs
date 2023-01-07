use bevy::prelude::*;

mod objects;

#[derive(Debug)]
pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(objects::ObjectsPlugin);
    }
}
