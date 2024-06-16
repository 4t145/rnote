use bevy::input::common_conditions::*;
use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::tools::{Tool, ToolBox};
#[derive(Component)]
pub struct Global2DCamera;
const CAMARA_INITIAL_TRANSFORM: Transform = Transform::from_xyz(0.0, 0.0, 100.0);
const CAMERA_ZOOM_LINE_SPEED: f32 = 0.1;
const CAMERA_ZOOM_PIXEL_SPEED: f32 = 0.001;
const CAMERA_ZOOM_RANGE: std::ops::Range<f32> = 0.2..5.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (zoom_scale, handle_drag.run_if(camera_control_condition)),
        );
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: CAMARA_INITIAL_TRANSFORM,
            ..default()
        },
        Global2DCamera,
    ));
}

fn camera_control_condition(
    tool_box: Res<ToolBox>,
    kbd_input: Res<ButtonInput<MouseButton>>,
) -> bool {
    if !kbd_input.any_pressed([MouseButton::Middle, MouseButton::Left]) {
        return false;
    }
    match tool_box.current_tool() {
        Some(Tool::Picker(p)) => !p.picked(),
        None => true,
        _ => false,
    }
}

fn handle_drag(
    mut query_camera_transform: Query<&mut Transform, With<Global2DCamera>>,
    query_camera_projection: Query<&OrthographicProjection, With<Global2DCamera>>,
    mut evr_mouse: EventReader<CursorMoved>,
) {
    let mut transform = query_camera_transform.single_mut();
    let projection = query_camera_projection.single();
    for ev in evr_mouse.read() {
        if let Some(delta) = ev.delta {
            transform.translation.x -= projection.scale * delta.x;
            transform.translation.y += projection.scale * delta.y;
        }
    }
}

fn zoom_scale(
    mut query_camera: Query<&mut OrthographicProjection, With<Global2DCamera>>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    let mut projection = query_camera.single_mut();
    for ev in evr_scroll.read() {
        match ev.unit {
            bevy::input::mouse::MouseScrollUnit::Line => {
                projection.scale = (projection.scale * (1.0 - ev.y * CAMERA_ZOOM_LINE_SPEED))
                    .clamp(CAMERA_ZOOM_RANGE.start, CAMERA_ZOOM_RANGE.end);
            }
            bevy::input::mouse::MouseScrollUnit::Pixel => {
                projection.scale = (projection.scale * (1.0 - ev.y * CAMERA_ZOOM_PIXEL_SPEED))
                    .clamp(CAMERA_ZOOM_RANGE.start, CAMERA_ZOOM_RANGE.end);
            }
        }
    }
}
