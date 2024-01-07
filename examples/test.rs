use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_2d_polyline::Polyline2d;
use bevy_pancam::{PanCamPlugin, PanCam};

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins((DefaultPlugins, PanCamPlugin::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mat = materials.add(Color::RED.into());
    let points = vec![
        [0.0, 0.0, 0.0],
        [150.0, 0.0, 0.0],
        [150.0, -50.0, 0.0],
        [200.0, 0.0, 0.0],
        [200.0, 100.0, 0.0],
        [0.0, 100.0, 0.0],
    ];
    let mesh = Polyline2d::new_closed(&points, 15.);

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        material: mat.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..Default::default()
    });

    commands.spawn(Camera2dBundle::default())
    .insert(PanCam {
        grab_buttons: vec![MouseButton::Middle],
        ..default()
    });
}