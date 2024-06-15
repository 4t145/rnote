use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod board;
mod camera;
mod debug;
mod mouse;
mod tools;
mod unit;
fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins);
    #[cfg(debug_assertions)] // debug/dev builds only
    {
        use bevy::diagnostic::LogDiagnosticsPlugin;
        app.add_plugins(LogDiagnosticsPlugin::default());
    }
    app.add_plugins(tools::ToolPlugin)
        .add_plugins(unit::UnitPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(debug::DebugPlugin)
        .add_plugins(board::BoardPlugin);
        
    app.run();
}
