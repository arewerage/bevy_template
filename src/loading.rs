use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::{ProgressCounter, ProgressPlugin};

use crate::GameState;

pub struct LoadingPlugin;

const MANUAL_DELAY_IN_SECS: f64 = 5.0;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_dynamic_collections::<StandardDynamicAssetCollection>(vec!["dynamic.assets"])
                .with_collection::<DebugAssets>()
                .with_collection::<ImageAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<FontAssets>(),
        )
        .add_plugin(ProgressPlugin::new(GameState::Loading).continue_to(GameState::MainMenu))
        .add_system(make_manual_loading_delay.run_in_state(GameState::Loading));
    }
}

#[derive(AssetCollection)]
struct DebugAssets {
    #[asset(path = "debug/big_texture.png")]
    _bit_texture: Handle<Image>,
    #[asset(path = "debug/big_texture_2.png")]
    _bit_texture_2: Handle<Image>,
}

#[derive(AssetCollection)]
struct ImageAssets {}

#[derive(AssetCollection)]
struct AudioAssets {}

#[derive(AssetCollection)]
struct FontAssets {}

fn make_manual_loading_delay(
    time: Res<Time>,
    progress: Res<ProgressCounter>,
    #[cfg(debug_assertions)]
    mut debugged: Local<bool>,
) {
    let ended = time.seconds_since_startup() > MANUAL_DELAY_IN_SECS;

    progress.manually_track((ended).into());

    #[cfg(debug_assertions)]
    if ended && !*debugged {
        info!(
            "Manual loading delay is completed! ({} seconds)",
            MANUAL_DELAY_IN_SECS
        );
        *debugged = true;
    }
}
