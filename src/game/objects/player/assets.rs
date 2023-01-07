use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Debug, AssetCollection, Resource)]
pub(in crate::game) struct PlayerAssets {
    #[asset(path = "models/player.glb#Scene0")]
    pub(super) scene: Handle<Scene>,
}
