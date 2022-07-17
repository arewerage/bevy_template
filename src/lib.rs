#[cfg(debug_assertions)]
mod debugging;
mod loading;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::AssetLoading)
            .add_plugin(loading::AssetLoadingPlugin);

        #[cfg(debug_assertions)]
        app.add_plugin(debugging::DebuggingPlugin);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    AssetLoading,
    SplashScreen,
    MainMenu,
    Gameplay,
}
