mod components;
mod states;
mod systems;

mod body;
mod space;

use bevy::prelude::*;
use systems::*;

pub struct SpacePlugin;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, fixed_update)
            .add_systems(Update, update);
    }
}
