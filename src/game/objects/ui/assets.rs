use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Debug)]
pub(in crate::game) struct UiAssets {
    #[asset(path = "fonts/prstart.ttf")]
    pub(super) font: Handle<Font>,
}
