use crate::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct GameAssetsPlugin;
impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ImageAssets>();
    }
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "ball.png")]
    pub ball: Handle<Image>,
}
