use bevy::{prelude::*, sprite::Mesh2dHandle};

use super::*;

#[derive(Bundle)]
pub struct Polyline2dBundle {
    pub polyline: Polyline2d,
    pub material: Handle<ColorMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: VisibilityBundle,
    pub mesh: Mesh2dHandle,
}

impl Default for Polyline2dBundle {
    fn default() -> Self {
        Polyline2dBundle {
            polyline: Polyline2d::default(),
            material: DEFAULT_MATERIAL_HANDLE,
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: VisibilityBundle::default(),
            mesh: Mesh2dHandle::default(),
        }
    }
}