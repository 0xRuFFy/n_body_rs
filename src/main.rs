mod components;
mod plugins;
mod resources;
mod systems;

mod ui;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use plugins::CustomDefaultPlugin;
use systems::{setup, testing_exit};
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(CustomDefaultPlugin)
        .add_systems(PreStartup, setup)
        .add_plugins((FrameTimeDiagnosticsPlugin, UiPlugin))
        .add_systems(Update, testing_exit)
        .run();
}
