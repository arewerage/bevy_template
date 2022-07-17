use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::SplashScreen)
                .with_dynamic_asset_collection_file("dynamic.assets")
                .with_collection::<ImageAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<FontAssets>(),
        );
    }
}

#[derive(AssetCollection)]
struct ImageAssets {}

#[derive(AssetCollection)]
struct AudioAssets {}

#[derive(AssetCollection)]
struct FontAssets {}
