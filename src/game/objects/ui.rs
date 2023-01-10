mod assets;
mod constants;
mod elapsed;
mod player_dead;
mod player_living;

use crate::state::AppState;

pub(in crate::game) use self::assets::UiAssets;
use self::elapsed::Elapsed;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Debug)]
pub(super) struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, setup)
            .add_plugin(player_living::PlayerLivingUiPlugin)
            .add_plugin(player_dead::PlayerDeadUiPlugin);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(Elapsed::default());
}
