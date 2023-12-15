use bevy::prelude::*;

#[derive(Resource)]
pub(super) struct Space;

impl Space {}

impl Default for Space {
    fn default() -> Self {
        Self {}
    }
}
