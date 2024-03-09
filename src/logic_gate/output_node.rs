use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{ColorPallet, MaterialHandles, MeshHandles, MeshType};

use super::value::Value;

#[derive(Component)]
pub struct OutputNode {
    pub endpoints: Vec<Entity>,
}

impl OutputNode {
    pub fn default() -> OutputNode {
        OutputNode { endpoints: vec![] }
    }
}

#[derive(Bundle)]
pub struct OutputNodeBundle {
    value: Value,
    output_node: OutputNode,
    sprite: MaterialMesh2dBundle<ColorMaterial>,
}

impl OutputNodeBundle {
    pub fn new(
        mesh_handle_library: &Res<MeshHandles>,
        material_handle_library: &Res<MaterialHandles>,
    ) -> OutputNodeBundle {
        let output_node = OutputNode::default();
        let mesh_handle = mesh_handle_library[MeshType::Circle].clone();
        let material_handle = material_handle_library[ColorPallet::FalseColor].clone();

        let output_bundle = OutputNodeBundle {
            value: Value::default(),
            output_node,
            sprite: MaterialMesh2dBundle {
                mesh: mesh_handle,
                material: material_handle,
                ..default()
            },
        };

        output_bundle
    }
}
