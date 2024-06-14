use bevy::prelude::*;
#[derive(Component)]
pub struct Board;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_board);
    }
}
pub fn setup_board(
    mut commands: Commands,

    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh = Mesh::from(shape::Quad::new(Vec2::new(100.0, 100.0)));
    let mesh_handle = meshes.add(mesh);

    let material = materials.add(Color::ANTIQUE_WHITE);
    // create our UI root node
    // this is the wrapper/container for the text
    let board = commands
        .spawn((
            Board,
            ColorMesh2dBundle {
                // 设置平面的变换属性，如位置、缩放等
                transform: Transform::from_scale(Vec3::splat(1.0)),
                mesh: mesh_handle.into(),
                material: material.clone(),
                // 设置平面的颜色
                ..Default::default()
            },
        ))
        .id();
}
