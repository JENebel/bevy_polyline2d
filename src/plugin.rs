use bevy::{prelude::*, sprite::Mesh2dHandle};

use super::*;

pub struct FlexLine2dPlugin;

impl Plugin for FlexLine2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, update_lines);

        app.world_mut().resource_mut::<Assets<ColorMaterial>>().insert(&BASE_MATERIAL_HANDLE, ColorMaterial::from_color(bevy::color::palettes::basic::WHITE));
    }
}

fn update_lines(
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&FlexLine, &mut Mesh2dHandle), Changed<FlexLine>>,
) {
    for (poly, mut mesh) in query.iter_mut() {
        mesh.0 = meshes.add(poly.make_mesh());
    }
}