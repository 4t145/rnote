//! Debug information
use bevy::prelude::*;
mod fps;
mod camera_info;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, fps::setup_counter)
            .add_systems(Update, (fps::counter_showhide, fps::text_update_system));
    }
}
