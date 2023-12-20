use super::body::Body;
use bevy::prelude::*;

#[derive(Resource)]
pub(super) struct Space {
    bodies: Vec<Entity>, // TODO: use a sparse quadtree instead
}

impl Space {
    pub fn fixed_update(&mut self, dt: f32, bodies: &mut Query<&mut Body>) {
        for entity in &self.bodies {
            if let Ok(body) = bodies.get(*entity) {
                let mut force = Vec2::ZERO;
                for other_entity in &self.bodies {
                    if *entity == *other_entity {
                        continue;
                    }
                    if let Ok(other_body) = bodies.get(*other_entity) {
                        force += body.force_between(&other_body);
                    }
                }
                let mut body = bodies.get_mut(*entity).unwrap();
                body.apply_force(force);
                body.fixed_update(dt);
            }
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

    pub fn add_body(
        &mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        body: Body,
    ) {
        self.bodies.push(body.spawn(commands, meshes, materials));
    }
}

impl Default for Space {
    fn default() -> Self {
        Self {
            bodies: Vec::new(),
        }
    }
}
