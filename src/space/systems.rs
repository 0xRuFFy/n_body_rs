use super::body::{Body, BodyBuilder};
use super::space::Space;
use bevy::prelude::*;
use rand_distr::{Distribution, LogNormal};

const START_SPEED: f32 = 10.0;
const TEST_SYSTEM: [(Vec2, Vec2, f32); 9] = [
    (Vec2::new(0.0, 0.0), Vec2::ZERO, 2024.0),
    (Vec2::new(1200.0, 1400.0), Vec2::new(-1.0, 0.0), 20.0),
    (Vec2::new(1000.0, 0.0), Vec2::new(0.0, 1.0), 16.0),
    (Vec2::new(1250.0, -1600.0), Vec2::new(0.0, 1.0), 14.0),
    (Vec2::new(0.0, -1750.0), Vec2::new(1.0, 0.0), 12.0),
    (Vec2::new(-1000.0, -900.0), Vec2::new(1.0, 0.0), 10.0),
    (Vec2::new(0.0, 1250.0), Vec2::new(-1.0, 0.0), 8.0),
    (Vec2::new(-1250.0, 1300.0), Vec2::new(0.0, -1.0), 6.0),
    (Vec2::new(-1400.0, 0.0), Vec2::new(0.0, -1.0), 4.0),
];

pub(super) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let normal = LogNormal::new(2.0, 1.0).unwrap();
    let mut space = Space::default();
    for i in -15..=15 {
        for j in -15..=15 {
            space.add_body(
                &mut commands,
                &mut meshes,
                &mut materials,
                BodyBuilder::default()
                    .position(Vec2::new(i as f32 * 50.0, j as f32 * 50.0))
                    .mass((normal.sample(&mut rng) as f32).abs() + 1.0)
                    .build(),
            )
        }
    }

    // for (p, v, m) in TEST_SYSTEM {
    //     space.add_body(
    //         &mut commands,
    //         &mut meshes,
    //         &mut materials,
    //         BodyBuilder::default()
    //             .position(p)
    //             .velocity(v * START_SPEED)
    //             .mass(m)
    //             .build(),
    //     )
    // }

    // space.add_body(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     BodyBuilder::default()
    //         .position(Vec2::new(0.0, 0.0))
    //         .mass(5000.0)
    //         .build(),
    // );
    // for i in 0..100 {
    //     space.add_body(
    //         &mut commands,
    //         &mut meshes,
    //         &mut materials,
    //         BodyBuilder::default()
    //             .position(Vec2::new(1000.0, -1000.0 + i as f32 * 20.0))
    //             .velocity(Vec2::new(-1.0 * START_SPEED, 0.0))
    //             .mass((normal.sample(&mut rng) as f32).abs() + 1.0)
    //             .build(),
    //     );
    // }

    commands.insert_resource(space);
}

pub(super) fn fixed_update(
    time: Res<Time<Fixed>>,
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
