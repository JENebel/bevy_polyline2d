use bevy::{color::palettes, prelude::*};
use bevy_flexline_2d::*;
use bevy_pancam::{PanCamPlugin, PanCam};

pub(crate) const BLUE_MATERIAL_HANDLE: Handle<ColorMaterial> = Handle::weak_from_u128(0xf274befa6c0e7f11d40d8931715303ac);

#[derive(Component)]
struct RotatingObject;

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "BMPoly".to_string(),
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((PanCamPlugin::default(), FlexLine2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, input_system);
    app.world_mut().resource_mut::<Assets<ColorMaterial>>().insert(&BLUE_MATERIAL_HANDLE, ColorMaterial::from_color(bevy::color::palettes::basic::BLUE));
    app.run();
}

fn setup(
    mut commands: Commands,
) {
    let points = vec![
        /*Vec2::new(0.0, 0.0),
        Vec2::new(150.0, 0.0),
        Vec2::new(150.0, -50.0),
        Vec2::new(200.0, 0.0),
        Vec2::new(200.0, 100.0),
        Vec2::new(0.0, 100.0),*/

        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 100.0),
        Vec2::new(100.0, 0.0),
    ];

    /*commands.spawn(Polyline2dBundle {
        polyline: FlexPath::new(
            points.clone(),
            10.,
            bevy_polyline2d::Alignment::Center,
            bevy_polyline2d::CornerStyle::Rounded { radius: 15., resolution: 24 },
            true
        ),
        ..Default::default()
    }).insert(RotatingObject);*/

    let mut transparent = Color::srgb(0., 0., 0.);
    transparent.set_alpha(0.);

    commands.spawn(FlexLine2dBundle {
        
        polyline: FlexLine::new(
            points.clone(),
            10.,
            Alignment::Center,
            CornerStyle::Rounded { radius: 0., resolution: 25 },
            ConnectionStyle::Unconnected,
            LineColor::GradientAcross{
                left: palettes::css::BLUE.into(), 
                right: transparent
            },
        ),
        ..Default::default()
    }).insert(RotatingObject).insert(Transform::from_translation(Vec3::new(0., 0., 1.)));

    commands.spawn(Camera2dBundle::default()).insert(PanCam {
        grab_buttons: vec![MouseButton::Right, MouseButton::Middle, MouseButton::Left],
        ..default()
    });
}

fn input_system(
    buttons: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut bevy_flexline_2d::FlexLine, &mut Transform)>
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
            if polyline.alignment == Alignment::LeftSide {
                polyline.alignment = Alignment::Center;
            } else {
                polyline.alignment = Alignment::LeftSide;
            }
        }

        if buttons.just_pressed(KeyCode::KeyC) {
            if polyline.connection_style == ConnectionStyle::Connected {
                polyline.connection_style = ConnectionStyle::Unconnected;
            } else {
                polyline.connection_style = ConnectionStyle::Connected;
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