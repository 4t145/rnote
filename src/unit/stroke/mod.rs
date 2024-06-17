use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    math,
    prelude::*,
};

use crate::{
    board::Board,
    camera::Global2DCamera,
    time::LastUpdate,
    tools::{picker::region::Region, Tool, ToolBox},
};

use super::{Active, Rendered, Unit};

pub enum DrawingStatus {
    Created,
    Recording,
    Finished,
}

#[derive(Component, Debug, Default)]
pub struct StrokeGroup {
    pub strokes: Vec<Stroke>,
    pub active_stroke: Option<Stroke>,
}

impl StrokeGroup {
    pub fn new() -> Self {
        Self::default()
    }

}

#[derive(Debug, Default)]
pub struct Stroke {
    pub measurements: Vec<PointMeasurement>,
}
impl Stroke {

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
    mut stroke_query: Query<
        (
            Entity,
            &mut StrokeGroup,
            &mut LastUpdate,
            &mut Region,
            &GlobalTransform,
        ),
        With<Active>,
    >,
    board: Query<(Entity, &GlobalTransform), With<Board>>,
    camera: Query<(&Camera, &GlobalTransform), With<Global2DCamera>>,
) {
    let (board_entity, board_gt) = board.single();
    let (camera, camera_gt) = camera.single();
    let Some(Tool::Brush {}) = tool_box.current_tool() else {
        return;
    };
    // 1. how many active strokes are there?
    let mut active_strokes = stroke_query.iter_mut().collect::<Vec<_>>();
    // 1.1 if there is no active stroke, and the left mouse button is just pressed, create a new stroke
    if active_strokes.is_empty() && mouse_button.just_pressed(MouseButton::Left) {
        info!("creating_stroke_group start");
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
                    StrokeGroup::new(),
                    Active,
                    Unit {
                        layer: 0,
                    },
                    Region::from_point(Vec2::default()),
                    LastUpdate::now(),
                    SpatialBundle {
                        transform,
                        ..Default::default()
                    },
                ))
                .set_parent(board_entity)
                .id();
            info!(
                "creating_stroke spawned a new stroke entity with id {:?}",
                id
            );
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
        let (id, mut stroke_group, mut last_update, mut region, gt) = active_strokes.pop().unwrap();
        const STICKY_DURATION: std::time::Duration = std::time::Duration::from_secs(3);

        if mouse_button.pressed(MouseButton::Left) {
            if !cursor_moved_events.is_empty() {
                last_update.update();
            }
            let translation = gt.translation();
            for event in cursor_moved_events.read() {
                let Some(world_p) = camera.viewport_to_world_2d(camera_gt, event.position) else {
                    warn!("creating_stroke add point failed, no world point found");
                    continue;
                };
                let current_stroke = stroke_group.active_stroke.get_or_insert(Default::default());
                let point = Vec2::new(world_p.x - translation.x, world_p.y - translation.y);
                current_stroke
                    .measurements
                    .push(PointMeasurement::new_point(point));
                region.rect = region.rect.union_point(point);
            }
        } else {
            let finished = stroke_group.active_stroke.take();
            stroke_group.strokes.extend(finished);
            last_update.update();
            debug!("creating_stroke finished");
        }
        if last_update.0.elapsed() > STICKY_DURATION {
            if let Some(last_stroke) = stroke_group.active_stroke.take() {
                if !last_stroke.measurements.is_empty() {
                    stroke_group.strokes.push(last_stroke);
                }
            }
            commands.entity(id).remove::<Active>();
            debug!("creating_stroke_group finished");
        }
    } else {
        // there is more than one active stroke, just inactivate all of them
        for (id, ..) in active_strokes {
            warn!("creating_stroke_group {id:?} finished with conflict");
            commands.entity(id).remove::<Active>();
        }
    }
}

pub fn render_strokes_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &StrokeGroup), (Without<Active>, Without<Rendered>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, stroke_group) in query.iter() {
        info!("rendering_stroke start");
        let mesh = Mesh::from(bevy::math::prelude::Rectangle::new(10.0, 10.0));
        let handle = materials.add(Color::PURPLE);
        for stroke in &stroke_group.strokes {
            for measurement in &stroke.measurements {
                let mesh_handle = meshes.add(mesh.clone());
                commands
                    .spawn(ColorMesh2dBundle {
                        mesh: mesh_handle.into(),
                        material: handle.clone(),
                        transform: Transform::from_translation(Vec3::new(
                            measurement.point.x,
                            measurement.point.y,
                            2.0,
                        )),
                        ..Default::default()
                    })
                    .set_parent(entity);
            }
        }
        commands.entity(entity).insert(Rendered);
        info!("rendering_stroke finished");
    }
}
