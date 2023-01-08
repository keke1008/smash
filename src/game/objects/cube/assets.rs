use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Debug)]
pub(in crate::game) struct CubeAssets {
    #[asset(path = "models/cube/wooden.glb#Scene0")]
    pub(super) wooden: Handle<Scene>,
    #[asset(path = "models/cube/stone.glb#Scene0")]
    pub(super) stone: Handle<Scene>,
    #[asset(path = "models/cube/metal.glb#Scene0")]
    pub(super) metal: Handle<Scene>,
}
