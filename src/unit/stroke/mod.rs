use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    math,
    prelude::*,
};

use crate::{
    board::Board,
    tools::{Tool, ToolBox},
};

use super::{Active, Rendered, Unit};
#[derive(Component, Debug)]
pub struct Stroke {
    pub points: Vec<PointMeasurement>,
}

#[derive(Debug)]
pub struct PointMeasurement {
    pub point: Vec2,
    pub press: Option<f32>,
}

impl PointMeasurement {
    pub fn new_point(point: Vec2) -> Self {
        Self { point, press: None }
    }
    pub fn with_press(mut self, press: f32) -> Self {
        self.press = Some(press);
        self
    }
}

pub fn stroke_record_system(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    tool_box: Res<ToolBox>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mb_input_events: EventReader<MouseButtonInput>,
    mut stroke_query: Query<(&mut Stroke, Entity), With<Active>>,
    mut board: Query<(Entity, &Board)>,
) {
    let (board_entity, board) = board.single();
    let Some(Tool::Brush {}) = tool_box.current_tool() else {
        return;
    };

    let mut points = Vec::new();
    for event in cursor_moved_events.read() {
        if mouse_button.pressed(MouseButton::Left) {
            points.push(PointMeasurement::new_point(event.position));
        }
    }
    if let Ok((mut stroke, id)) = stroke_query.get_single_mut() {
        stroke.points.extend(points);
        if mouse_button.just_released(MouseButton::Left) {
            commands.entity(id).remove::<Active>();
            info!("created_stroke");
        }
    } else {
        info!("creating_stroke");
        if mouse_button.just_pressed(MouseButton::Left) {
            let move_start_point = points.
            commands
                .spawn((
                    Stroke { points: vec![] },
                    Active,
                    Unit,
                    SpatialBundle {
                        transform: mouse_button.
                        ..Default::default()
                    },
                ))
                .set_parent(board_entity);
        }
    }
}

// 渲染笔画的系统
pub fn render_strokes_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &Stroke, &GlobalTransform), (Without<Active>, Without<Rendered>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, stroke, gt) in query.iter() {
        let mesh = Mesh::from(bevy::math::prelude::Circle::new(5.0));
        let handle = materials.add(Color::PURPLE);
        for measure in &stroke.points {
            let mesh_handle = meshes.add(mesh.clone());
            commands
                .spawn(ColorMesh2dBundle {
                    mesh: mesh_handle.into(),
                    material: handle.clone_weak(),
                    transform: GlobalTransform::from_translation(Vec3::new(
                        measure.point.x,
                        measure.point.y,
                        0.0,
                    ))
                    .reparented_to(gt),
                    ..Default::default()
                })
                .set_parent(entity);
        }
        commands.entity(entity).insert(Rendered);
    }
}
