use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod board;
mod camera;
mod debug;
pub mod mouse;
fn main() {
    let mut app = App::new();
    app.add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(DefaultPlugins);
    #[cfg(debug_assertions)] // debug/dev builds only
    {
        use bevy::diagnostic::LogDiagnosticsPlugin;
        app.add_plugins(LogDiagnosticsPlugin::default());
    }
    app.add_plugins(camera::CameraPlugin)
        .add_plugins(debug::DebugPlugin)
        .add_plugins(board::BoardPlugin);
    app.run();
}
