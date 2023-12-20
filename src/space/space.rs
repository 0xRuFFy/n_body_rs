use super::body::Body;
use bevy::prelude::*;

#[derive(Resource)]
pub(super) struct Space;

impl Space {
    pub fn fixed_update(&mut self, dt: f32, bodies: &mut Query<&mut Body>) {
        for mut body in bodies.iter_mut() {
            // TODO: use apply_force here ...

            body.fixed_update(dt);
        }
    }

    pub fn update(&mut self, dt: f32, bodies: &mut Query<(&mut Body, &mut Transform)>) {
        for (mut body, mut transform) in bodies.iter_mut() {
            body.update(dt);
            if let Some(position) = body.position() {
                transform.translation = position.extend(0.0);
            }
        }
    }
}

impl Default for Space {
    fn default() -> Self {
        Self {}
    }
}
