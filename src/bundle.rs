use bevy::{prelude::*, sprite::Mesh2dHandle};

use super::*;

#[derive(Bundle)]
pub struct FlexLine2dBundle {
    pub polyline: FlexLine,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub material: Handle<ColorMaterial>,
    pub mesh: Mesh2dHandle,
}

impl Default for FlexLine2dBundle {
    fn default() -> Self {
        FlexLine2dBundle {
            polyline: FlexLine::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            material: BASE_MATERIAL_HANDLE,
            mesh: Mesh2dHandle::default(),
        }
    }
}