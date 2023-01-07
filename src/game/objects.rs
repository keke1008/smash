mod player;
mod stage;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::state::AppState;

pub(in crate::game) use self::player::PlayerAssets;

#[derive(Debug)]
pub(super) struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.add(stage::SpawnStage);
    commands.add(player::SpawnPlayer);
}
