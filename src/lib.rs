#[cfg(debug_assertions)]
mod debugging;
mod loading;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::Loading)
            .add_plugin(loading::LoadingPlugin);

        #[cfg(debug_assertions)]
        app.add_plugin(debugging::DebuggingPlugin);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Loading,
    MainMenu,
    Gameplay,
}
