use bevy::{prelude::*, sprite::Mesh2dHandle};

use super::*;

#[derive(Bundle)]
pub struct Polyline2dBundle {
    pub polyline: polyline2d::Polyline2d,
    pub material: Handle<ColorMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub mesh: Mesh2dHandle,
}

impl Default for Polyline2dBundle {
    fn default() -> Self {
        Polyline2dBundle {
            polyline: polyline2d::Polyline2d::default(),
            material: DEFAULT_MATERIAL_HANDLE,
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),

            mesh: Mesh2dHandle::default(),
        }
    }
}