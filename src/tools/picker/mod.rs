pub mod region;
use bevy::{input::keyboard::KeyboardInput, prelude::*, window::PrimaryWindow};
use region::Region;

use crate::{camera::Global2DCamera, unit::Unit};

use super::{Tool, ToolBox};

#[derive(Debug, Default)]
pub struct Picker {
    pub selected: Vec<Entity>,
}

impl Picker {
    #[inline]
    pub fn picked(&self) -> bool {
        !self.selected.is_empty()
    }
}

pub fn pick_unit_system(
    // these will panic if the resources don't exist
    mut tool_box: ResMut<ToolBox>,
    mut mouse_input: Res<ButtonInput<MouseButton>>,
    mut kbd_input: Res<ButtonInput<KeyCode>>,
    mut q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_unit: Query<(Entity, &GlobalTransform, &Region, &Unit)>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Global2DCamera>>,
) {
    let picker = match tool_box.current_tool_mut() {
        Some(Tool::Picker(p)) => p,
        _ => return,
    };
    let (camera, camera_gt) = q_camera.single();
    let window = q_window.single();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    if mouse_input.just_pressed(MouseButton::Left) {
        let mouse_position = match camera.viewport_to_world_2d(camera_gt, cursor_position) {
            Some(p) => p,
            None => return,
        };
        for (entity, gt, region, unit) in q_unit.iter_mut() {
            let base_position = gt.translation().truncate();
            let mouse_position = mouse_position - base_position;
            if region.rect.contains(mouse_position) {
                picker.selected.push(entity)
            }
        }
    }
}
