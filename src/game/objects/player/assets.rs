use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Debug, AssetCollection, Resource)]
pub(in crate::game) struct PlayerAssets {
    #[asset(path = "models/player.glb#Scene0")]
    pub(super) scene: Handle<Scene>,

    #[asset(path = "models/player.glb#Animation3")]
    pub(super) stay: Handle<AnimationClip>,
    #[asset(path = "models/player.glb#Animation2")]
    pub(super) run: Handle<AnimationClip>,
    #[asset(path = "models/player.glb#Animation0")]
    pub(super) left_punch: Handle<AnimationClip>,
    #[asset(path = "models/player.glb#Animation1")]
    pub(super) right_punch: Handle<AnimationClip>,
}
