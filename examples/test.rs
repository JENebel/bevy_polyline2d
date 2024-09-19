use bevy::prelude::*;
use bevy_polyline2d::{Align::*, FlexPath, Polyline2d, Polyline2dBundle, Polyline2dPlugin};
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
        Vec2::new(0.0, 0.0),
        Vec2::new(150.0, 0.0),
        Vec2::new(150.0, -50.0),
        Vec2::new(200.0, 0.0),
        Vec2::new(200.0, 100.0),
        Vec2::new(0.0, 100.0),
    ];

    let polyline = FlexPath::new(
        points,
        10.0,
        bevy_polyline2d::Alignment::RightSide,
        bevy_polyline2d::CornerStyle::Sharp,
        true
    );
    
    commands.spawn(Polyline2dBundle {
        polyline,
        ..Default::default()
    }).insert(RotatingObject);

    commands.spawn(Camera2dBundle::default())
    .insert(PanCam {
        grab_buttons: vec![MouseButton::Right, MouseButton::Middle, MouseButton::Left],
        ..default()
    });
}

fn input_system(
    buttons: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Polyline2d, &mut Transform)>
) {
    for (mut polyline, mut trans) in query.iter_mut() {
        if buttons.pressed(KeyCode::ArrowUp) {
            trans.translation.y += 1.0;
        }
        if buttons.pressed(KeyCode::ArrowDown) {
            trans.translation.y -= 1.0;
        }
        if buttons.pressed(KeyCode::ArrowRight) {
            trans.translation.x += 1.0;
        }
        if buttons.pressed(KeyCode::ArrowLeft) {
            trans.translation.x -= 1.0;
        }

        if buttons.just_pressed(KeyCode::KeyL) {
            if polyline.line_placement == Left {
                polyline.line_placement = Center;
            } else {
                polyline.line_placement = Left;
            }
        }

        if buttons.just_pressed(KeyCode::KeyC) {
            if polyline.closed {
                polyline.closed = false;
            } else {
                polyline.closed = true;
            }
        }

        if buttons.just_pressed(KeyCode::KeyQ) {
            polyline.width += 1.0;
        }
        if buttons.just_pressed(KeyCode::KeyW) {
            polyline.width -= 1.0;
        }
    }
}