use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use iyes_loopless::prelude::*;

use crate::GameState;

pub struct DebuggingPlugin;

impl Plugin for DebuggingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(WorldInspectorPlugin::default())
            .add_system(bevy::input::system::exit_on_esc_system)
            .add_system(change_game_state.run_if(n_key_just_pressed))
            .add_system(print_current_game_state);
    }
}

fn change_game_state(mut commands: Commands, game_state: Res<CurrentState<GameState>>) {
    match game_state.0 {
        GameState::AssetLoading => {
            commands.insert_resource(NextState(GameState::SplashScreen));
        }
        GameState::SplashScreen => {
            commands.insert_resource(NextState(GameState::MainMenu));
        }
        GameState::MainMenu => {
            commands.insert_resource(NextState(GameState::Gameplay));
        }
        GameState::Gameplay => {
            commands.insert_resource(NextState(GameState::AssetLoading));
        }
    }
}

fn n_key_just_pressed(keys: Res<Input<KeyCode>>) -> bool {
    keys.just_pressed(KeyCode::N)
}

fn print_current_game_state(game_state: Res<CurrentState<GameState>>) {
    if game_state.is_changed() {
        println!("Current GameState is {:?}", game_state.0);
    }
}
