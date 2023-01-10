mod assets;
mod constants;
mod groups;
mod input;
mod objects;
mod state;
mod systems;
mod tags;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::state::AppState;

use self::state::PlayerState;

#[derive(Debug)]
pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(PlayerState::NotInGame)
            .add_enter_system(AppState::InGame, update_player_state)
            .add_plugin(assets::GameAssetPlugin)
            .add_plugin(objects::ObjectsPlugin)
            .add_plugin(input::InputPlugin)
            .add_plugin(systems::SystemsPlugin);
    }
}

fn update_player_state(mut commands: Commands) {
    commands.insert_resource(NextState(PlayerState::Living));
}
