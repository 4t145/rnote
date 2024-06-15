use bevy::prelude::*;
mod stroke;
#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Active;
#[derive(Component)]
pub struct Rendered;
pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, stroke::stroke_record_system)
            .add_systems(Update, stroke::render_strokes_system);
    }
}
// Board -> Unit