use bevy::prelude::*;
use bevy_polyline2d::{Polyline2d, Align::*, Polyline2dBundle, Polyline2dPlugin};
use bevy_pancam::{PanCamPlugin, PanCam};

#[derive(Component)]
struct RotatingObject;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins((DefaultPlugins, PanCamPlugin::default(), Polyline2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, input_system)
        .run();
}

fn setup(
    mut commands: Commands,
) {
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
        line_placement: Left,
        width: 10.0,
    };
    
    commands.spawn(Polyline2dBundle {
        polyline,
        ..Default::default()
    }).insert(RotatingObject);

    commands.spawn(Camera2dBundle::default())
    .insert(PanCam {
        grab_buttons: vec![MouseButton::Middle],
        ..default()
    });
}

fn input_system(
    buttons: Res<Input<KeyCode>>,
    mut query: Query<(&mut Polyline2d, &mut Transform)>
) {
    for (mut polyline, mut trans) in query.iter_mut() {
        if buttons.pressed(KeyCode::Up) {
            trans.translation.y += 1.0;
        }
        if buttons.pressed(KeyCode::Down) {
            trans.translation.y -= 1.0;
        }
        if buttons.pressed(KeyCode::Right) {
            trans.translation.x += 1.0;
        }
        if buttons.pressed(KeyCode::Left) {
            trans.translation.x -= 1.0;
        }

        if buttons.just_pressed(KeyCode::L) {
            if polyline.line_placement == Left {
                polyline.line_placement = Center;
            } else {
                polyline.line_placement = Left;
            }
        }

        if buttons.just_pressed(KeyCode::C) {
            if polyline.closed {
                polyline.closed = false;
            } else {
                polyline.closed = true;
            }
        }

        if buttons.just_pressed(KeyCode::Q) {
            polyline.width += 1.0;
        }
        if buttons.just_pressed(KeyCode::W) {
            polyline.width -= 1.0;
        }
    }
}