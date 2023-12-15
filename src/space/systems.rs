use super::body::BodyBuilder;
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
