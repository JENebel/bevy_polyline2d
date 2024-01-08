use bevy::{prelude::*, sprite::Mesh2dHandle};

use super::*;

pub struct Polyline2dPlugin;

impl Plugin for Polyline2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, update_polys);
        app.world.resource_mut::<Assets<ColorMaterial>>().insert(DEFAULT_MATERIAL_HANDLE, Color::WHITE.into());
    }
}

fn update_polys(
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&Polyline2d, &mut Mesh2dHandle), Changed<Polyline2d>>,
) {
    //println!("update_polys");
    for (poly, mut mesh) in query.iter_mut() {
        mesh.0 = meshes.add(poly.make_mesh());
    }
}