use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{ColorPallet, Interactable, MaterialHandles, MeshHandles, MeshType};

use super::value::Value;

#[derive(Component)]
pub struct InputNode {
    pub source: Option<Entity>,
}

impl InputNode {
    pub fn default() -> InputNode {
        InputNode { source: None }
    }
}

#[derive(Bundle)]
pub struct InputNodeBundle {
    value: Value,
    input_node: InputNode,
    sprite: MaterialMesh2dBundle<ColorMaterial>,
    interactable: Interactable,
}

impl InputNodeBundle {
    pub fn new(
        mesh_handle_library: &Res<MeshHandles>,
        material_handle_library: &Res<MaterialHandles>,
    ) -> InputNodeBundle {
        let input_node = InputNode::default();
        let mesh_handle = mesh_handle_library[MeshType::Circle].clone();
        let material_handle = material_handle_library[ColorPallet::FalseColor].clone();

        InputNodeBundle {
            value: Value::default(),
            input_node,
            interactable: Interactable::default(),
            sprite: MaterialMesh2dBundle {
                mesh: mesh_handle,
                material: material_handle,
                ..default()
            },
        }
    }
}
