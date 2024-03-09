use std::ops::Index;

use bevy::prelude::*;
use enum_map::{enum_map, Enum, EnumMap};

use crate::{logic_gate::value::Value, MaterialHandles};

#[derive(Debug, Enum, Clone, Copy)]
pub enum ColorPallet {
    ChipColor,
    BoardColor,
    TrueColor,
    FalseColor,
    InterfaceColor,
    InterfaceColorHighlight,
}

#[derive(Debug, Resource)]
pub struct Colors {
    pub colors: EnumMap<ColorPallet, Color>,
}

// allow indexing in to Colors
impl Index<ColorPallet> for Colors {
    type Output = Color;

    fn index(&self, index: ColorPallet) -> &Self::Output {
        &self.colors[index]
    }
}

impl Default for Colors {
    fn default() -> Self {
        Colors {
            colors: enum_map! {
                ColorPallet::ChipColor => Color::rgb(0.1,0.1,0.1),
                ColorPallet::BoardColor => Color::rgb(0.2,0.2,0.2),
                ColorPallet::TrueColor => Color::rgb(0.2, 0.4, 0.2),
                ColorPallet::FalseColor => Color::rgb(0.7, 0.3, 0.3),
                ColorPallet::InterfaceColor => Color::rgb(0.1,0.1,0.1),
                ColorPallet::InterfaceColorHighlight => Color::rgb(0.4,0.4,0.4),
            },
        }
    }
}

pub fn color_system(
    mut input_query: Query<(&mut Handle<ColorMaterial>, &Value), Changed<Value>>,

    material_handles: Res<MaterialHandles>,
) {
    for (mut material, node) in input_query.iter_mut() {
        let color = if node.0 {
            ColorPallet::TrueColor
        } else {
            ColorPallet::FalseColor
        };

        *material = material_handles[color].clone();
    }
}
