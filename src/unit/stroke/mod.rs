use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    math,
    prelude::*,
};

use crate::{
    board::Board,
    camera::Global2DCamera,
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
    mut stroke_query: Query<(Entity, &mut Stroke, &GlobalTransform), With<Active>>,
    mut board: Query<(Entity, &Board, &GlobalTransform)>,
    mut camera: Query<(&Camera, &GlobalTransform), With<Global2DCamera>>,
) {
    let (board_entity, board, board_gt) = board.single();
    let (camera, camera_gt) = camera.single();
    let Some(Tool::Brush {}) = tool_box.current_tool() else {
        return;
    };
    // 1. how many active strokes are there?
    let mut active_strokes = stroke_query.iter_mut().collect::<Vec<_>>();
    // 1.1 if there is no active stroke, and the left mouse button is just pressed, create a new stroke
    if active_strokes.is_empty() && mouse_button.just_pressed(MouseButton::Left) {
        info!("creating_stroke start");
        let translation = board_gt.translation();
        if let Some(last_moved) = cursor_moved_events.read().last() {
            let Some(world_p) = camera.viewport_to_world_2d(camera_gt, last_moved.position) else {
                warn!("creating_stroke failed, no world point found");
                return;
            };
            let transform = Transform::from_translation(Vec3::new(
                world_p.x - translation.x,
                world_p.y - translation.y,
                1.0,
            ));
            let id = commands
                .spawn((
                    Stroke {
                        points: vec![PointMeasurement::new_point(Vec2::new(0.0, 0.0))],
                    },
                    Active,
                    Unit,
                    SpatialBundle {
                        transform,
                        ..Default::default()
                    },
                ))
                .set_parent(board_entity)
                .id();
            info!("creating_stroke spawned a new stroke entity with id {:?}", id);
            if !mouse_button.pressed(MouseButton::Left) {
                info!(
                    "creating_stroke finished in one poll, from {:?} to {:?}",
                    world_p, world_p
                );
                commands.entity(id).remove::<Active>();
            }
        } else {
            warn!("creating_stroke failed, no cursor moved event found");
        }
    } else if active_strokes.len() == 1 {
        info!("creating_stroke continue");
        let (id, mut stroke, gt) = active_strokes.pop().unwrap();
        if mouse_button.pressed(MouseButton::Left) {
            let translation = gt.translation();
            for event in cursor_moved_events.read() {
                let Some(world_p) = camera.viewport_to_world_2d(camera_gt, event.position) else {
                    warn!("creating_stroke add point failed, no world point found");
                    continue;
                };

                stroke.points.push(PointMeasurement::new_point(Vec2::new(
                    world_p.x - translation.x,
                    world_p.y - translation.y,
                )));
            }
        } else {
            info!(
                "creating_stroke finished, from {:?} to {:?}",
                stroke.points.first(),
                stroke.points.last()
            );
            commands.entity(id).remove::<Active>();
        }
    } else {
        // there is more than one active stroke, just inactivate all of them
        for (id, stroke, _) in active_strokes {
            info!(
                "creating_stroke finished, from {:?} to {:?}",
                stroke.points.first(),
                stroke.points.last()
            );
            commands.entity(id).remove::<Active>();
        }
    }
}

pub fn render_strokes_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &Stroke), (Without<Active>, Without<Rendered>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, stroke) in query.iter() {
        info!("rendering_stroke start");
        let mesh = Mesh::from(bevy::math::prelude::Rectangle::new(10.0, 10.0));
        let handle = materials.add(Color::PURPLE);
        for measure in &stroke.points {
            let mesh_handle = meshes.add(mesh.clone());
            commands
                .spawn(ColorMesh2dBundle {
                    mesh: mesh_handle.into(),
                    material: handle.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        measure.point.x,
                        measure.point.y,
                        2.0,
                    )),
                    ..Default::default()
                })
                .set_parent(entity);
        }
        commands.entity(entity).insert(Rendered);
        info!("rendering_stroke finished");
    }
}
