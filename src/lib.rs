mod polyline2d;
mod plugin;
mod bundle;
mod flex_path;
mod vector_extensions;

use bevy::{sprite::ColorMaterial, asset::Handle};

pub(crate) const DEFAULT_MATERIAL_HANDLE: Handle<ColorMaterial> = Handle::weak_from_u128(0xf724befa6c0e7f11d40d8931715303ac);

pub use crate::{
    plugin::Polyline2dPlugin, 
    polyline2d::{Polyline2d, Align},
    bundle::Polyline2dBundle,
};