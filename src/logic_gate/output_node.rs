use crate::*;

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
    interactable: Interactable,
}

impl OutputNodeBundle {
    pub fn new(
        mesh_handle_library: &Res<MeshHandles>,
        material_handle_library: &Res<MaterialHandles>,
    ) -> OutputNodeBundle {
        let output_node = OutputNode::default();
        let mesh_handle = mesh_handle_library[MeshType::Circle].clone();
        let material_handle = material_handle_library[ColorPallet::FalseColor].clone();

        OutputNodeBundle {
            value: Value::default(),
            output_node,
            sprite: MaterialMesh2dBundle {
                mesh: mesh_handle,
                material: material_handle,
                ..default()
            },
            interactable: Interactable::default(),
        }
    }
}
