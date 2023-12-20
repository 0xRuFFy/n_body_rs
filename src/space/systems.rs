use super::body::{Body, BodyBuilder};
use super::space::Space;
use bevy::prelude::*;

pub(super) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Space::default());
    BodyBuilder::default()
        .position(Vec2::new(0.0, 0.0))
        .mass(64.)
        .build()
        .spawn(&mut commands, &mut meshes, &mut materials);
}

pub(super) fn fixed_update(
    time: Res<Time>,
    mut space: ResMut<Space>,
    mut bodies: Query<&mut Body>,
) {
    let dt = time.delta_seconds();
    space.fixed_update(dt, &mut bodies);
}

pub(super) fn update(
    time: Res<Time>,
    mut space: ResMut<Space>,
    mut bodies: Query<(&mut Body, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    space.update(dt, &mut bodies);
}
