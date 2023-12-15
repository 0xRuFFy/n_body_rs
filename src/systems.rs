use crate::components::MainCamera;
use crate::resources::ShowFps;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowMode};

pub const PIXELS_PER_UNIT: f32 = 1.0;
const CLEAR_COLOR: Color = Color::rgb(0.035, 0., 0.070);
const RESOLUTION: (f32, f32) = (1280., 720.);

const SCROLL_SENSITIVITY: f32 = 0.01;

pub(super) fn setup(
    mut commands: Commands,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = window_query.single_mut();

    window.title = "Falling Sand".into();
    window.present_mode = PresentMode::AutoNoVsync;
    window.mode = WindowMode::Windowed;
    window.resolution = RESOLUTION.into();

    commands.insert_resource(ShowFps(true));

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 999.0),
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(CLEAR_COLOR),
            },
            projection: OrthographicProjection {
                scaling_mode: bevy::render::camera::ScalingMode::WindowSize(PIXELS_PER_UNIT),
                ..default()
            },
            ..default()
        },
        MainCamera,
    ));
}

pub(super) fn camera_control(
    // keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut motion_events: EventReader<MouseMotion>,
) {
    let mut camera = camera_query.single_mut();

    for event in scroll_events.iter() {
        match event.unit {
            MouseScrollUnit::Line => {
                let delta = event.y * SCROLL_SENSITIVITY;
                camera.scale.x = (camera.scale.x - delta).max(0.01);
                camera.scale.y = (camera.scale.y - delta).max(0.01);
            }
            _ => (),
        }
    }

    if !mouse_input.any_pressed([MouseButton::Left]) {
        motion_events.clear();
        return;
    }

    for event in motion_events.read() {
        camera.translation.x -= event.delta.x * time.delta_seconds() * 400.0 * camera.scale.x;
        camera.translation.y += event.delta.y * time.delta_seconds() * 400.0 * camera.scale.y;
    }
}

pub(super) fn exit(mut exit: EventWriter<bevy::app::AppExit>, key: Res<Input<KeyCode>>) {
    if key.just_pressed(KeyCode::Escape) {
        println!("ESC pressed, exiting...");
        exit.send(bevy::app::AppExit);
    }
}
