use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use iyes_loopless::prelude::*;
use iyes_progress::ProgressCounter;

use crate::GameState;

pub struct DebuggingPlugin;

impl Plugin for DebuggingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(WorldInspectorPlugin::default())
            .add_system(bevy::input::system::exit_on_esc_system)
            .add_system(change_game_state.run_if(n_key_just_pressed))
            .add_system(print_game_state)
            .add_system_to_stage(CoreStage::PostUpdate, print_loading_progress);
    }
}

fn change_game_state(mut commands: Commands, game_state: Res<CurrentState<GameState>>) {
    match game_state.0 {
        GameState::Loading => {
            commands.insert_resource(NextState(GameState::MainMenu));
        }
        GameState::MainMenu => {
            commands.insert_resource(NextState(GameState::Gameplay));
        }
        GameState::Gameplay => {
            commands.insert_resource(NextState(GameState::Loading));
        }
    }
}

fn n_key_just_pressed(keys: Res<Input<KeyCode>>) -> bool {
    keys.just_pressed(KeyCode::N)
}

fn print_game_state(game_state: Res<CurrentState<GameState>>) {
    if game_state.is_changed() {
        info!("Current GameState is {:?}", game_state.0);
    }
}

fn print_loading_progress(progress: Option<Res<ProgressCounter>>) {
    if let Some(progress) = progress {
        if progress.is_changed() {
            info!("Current progress: {:?}", progress.progress());
        }
    }
}
