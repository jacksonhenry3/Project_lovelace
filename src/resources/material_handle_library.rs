use bevy::prelude::*;
use enum_map::{enum_map, Enum, EnumMap};
use std::ops::Index;

#[derive(Debug, Enum, Clone, Copy)]
pub enum ColorPallet {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    White,
    Black,
}

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

// add all the colors in to a resource
pub(crate) fn initialize_colors(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let colors = enum_map! {
       ColorPallet::Red => materials.add(Color::rgb(1.0, 0.0, 0.0)),
       ColorPallet::Green => materials.add(Color::rgb(0.0, 1.0, 0.0)),
       ColorPallet::Blue => materials.add(Color::rgb(0.0, 0.0, 1.0)),
       ColorPallet::Yellow => materials.add(Color::rgb(1.0, 1.0, 0.0)),
       ColorPallet::Cyan => materials.add(Color::rgb(0.0, 1.0, 1.0)),
       ColorPallet::Magenta => materials.add(Color::rgb(1.0, 0.0, 1.0)),
       ColorPallet::White => materials.add(Color::rgb(1.0, 1.0, 1.0)),
       ColorPallet::Black => materials.add(Color::rgb(0.0, 0.0, 0.0)),
    };

    commands.insert_resource(MaterialHandles { materials: colors });
}
