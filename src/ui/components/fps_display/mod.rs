mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

pub(super) const FPS_COLOR: Color = Color::rgb(0.949, 0.706, 0.51);

pub struct FpsDisplayPlugin;

impl Plugin for FpsDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, update);
    }
}
