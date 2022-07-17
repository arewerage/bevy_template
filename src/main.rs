#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{default, App, ClearColor, Color, DefaultPlugins, WindowDescriptor};
use bevy_template::GamePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::GRAY))
        .insert_resource(WindowDescriptor {
            title: "Bevy Template".to_string(),
            width: 1280.0,
            height: 720.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
