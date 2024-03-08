use bevy::prelude::*;
use enum_map::EnumMap;
use std::ops::Index;

use super::colors::*;

#[derive(Debug, Resource)]
pub struct MaterialHandles {
    pub materials: EnumMap<ColorPallet, Handle<ColorMaterial>>,
}

// allow indexing in to Colors
impl Index<ColorPallet> for MaterialHandles {
    type Output = Handle<ColorMaterial>;

    fn index(&self, index: ColorPallet) -> &Self::Output {
        &self.materials[index]
    }
}

impl FromWorld for MaterialHandles {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let colors = Colors::default();
        let mut material_handles = EnumMap::<ColorPallet, Handle<ColorMaterial>>::default();

        for (color_pallet, color) in colors.colors.iter() {
            let handle = materials.add(ColorMaterial::from(*color));
            material_handles[color_pallet] = handle;
        }

        MaterialHandles {
            materials: material_handles,
        }
    }
}
