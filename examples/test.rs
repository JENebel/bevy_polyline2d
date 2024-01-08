use bevy::prelude::*;
use bevy_polyline2d::{Polyline2d, LinePlacement::*, Polyline2dBundle, Polyline2dPlugin};
use bevy_pancam::{PanCamPlugin, PanCam};

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins((DefaultPlugins, PanCamPlugin::default(), Polyline2dPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mat = materials.add(Color::RED.into());
    let points = vec![
        [0.0, 0.0, 0.0],
        [150.0, 0.0, 0.0],
        [150.0, -50.0, 0.0],
        [200.0, 0.0, 0.0],
        [200.0, 100.0, 0.0],
        [100.0, 100.0, 0.0],
        [0.0, 100.0, 0.0],
    ];

    let polyline = Polyline2d {
        path: points,
        closed: true,
        line_placement: LeftOf,
        width: 10.0,
    };
    
    commands.spawn(Polyline2dBundle {
        polyline,
        material: mat,
        ..Default::default()
    });

    commands.spawn(Camera2dBundle::default())
    .insert(PanCam {
        grab_buttons: vec![MouseButton::Middle],
        ..default()
    });
}