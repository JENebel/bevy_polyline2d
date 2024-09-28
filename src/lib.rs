mod plugin;
mod bundle;

//#[allow(dead_code)]
mod flex_line;
mod vector_utils;

pub(crate) const BASE_MATERIAL_HANDLE: Handle<ColorMaterial> = Handle::weak_from_u128(0xf724befa6c0e7f11d40d8931715303ac);

use bevy::{asset::Handle, sprite::ColorMaterial};

pub use crate::{
    plugin::FlexLine2dPlugin, 
    bundle::FlexLine2dBundle,
    flex_line::{
        FlexLine, CornerStyle, Alignment, 
        ConnectionStyle, LineColor
    },
};