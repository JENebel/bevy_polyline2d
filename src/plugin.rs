use bevy::{prelude::*, sprite::Mesh2dHandle};

use super::*;

pub struct Polyline2dPlugin;

impl Plugin for Polyline2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, update_polys);
        app.world_mut().resource_mut::<Assets<ColorMaterial>>().insert(&DEFAULT_MATERIAL_HANDLE, ColorMaterial::from_color(bevy::color::palettes::basic::RED));
    }
}

fn update_polys(
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&FlexPath, &mut Mesh2dHandle), Changed<FlexPath>>,
) {
    //println!("update_polys");
    for (poly, mut mesh) in query.iter_mut() {
        mesh.0 = meshes.add(poly.make_mesh());
    }
}