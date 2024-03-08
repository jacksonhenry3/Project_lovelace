use bevy::{core_pipeline::core_2d::graph::input, prelude::*, sprite::MaterialMesh2dBundle};

use crate::{ColorPallet, MaterialHandles, MeshHandles, MeshType};

#[derive(Component)]
pub struct InputNode {
    pub value: bool,
    pub source: Option<Entity>,
}

impl InputNode {
    pub fn default() -> InputNode {
        InputNode {
            value: false,
            source: None,
        }
    }
}

#[derive(Bundle)]
pub struct InputNodeBundle {
    input_node: InputNode,
    sprite: MaterialMesh2dBundle<ColorMaterial>,
}

impl InputNodeBundle {
    pub fn new(
        mesh_handle_library: &Res<MeshHandles>,
        material_handle_library: &Res<MaterialHandles>,
    ) -> InputNodeBundle {
        let input_node = InputNode::default();
        let mesh_handle = mesh_handle_library[MeshType::Circle].clone();
        let material_handle = material_handle_library[ColorPallet::FalseColor].clone();

        let input_bundle = InputNodeBundle {
            input_node,
            sprite: MaterialMesh2dBundle {
                mesh: mesh_handle,
                material: material_handle,
                ..default()
            },
        };

        input_bundle
    }
}
