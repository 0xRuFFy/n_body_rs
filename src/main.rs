mod components;
mod plugins;
mod resources;
mod systems;

mod space;
mod ui;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use plugins::CustomDefaultPlugin;
use space::SpacePlugin;
use systems::*;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(CustomDefaultPlugin)
        .add_systems(PreStartup, setup)
        .add_plugins((FrameTimeDiagnosticsPlugin, UiPlugin, SpacePlugin))
        .add_systems(Update, (exit, camera_control))
        .run();
}
