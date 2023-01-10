use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::state::AppState;

use super::objects::{CubeAssets, PlayerAssets, UiAssets};

#[derive(Debug)]
pub(super) struct GameAssetPlugin;

impl Plugin for GameAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::InGame)
                .with_collection::<PlayerAssets>()
                .with_collection::<CubeAssets>()
                .with_collection::<UiAssets>(),
        );
    }
}
