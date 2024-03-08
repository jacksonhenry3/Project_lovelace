use bevy::sprite;
use bevy::utils::HashMap;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::logic_gate::{input_node, output_node};
use crate::sizes::NOT_GATE_WIDTH;
use crate::{sizes, z_height_manager, ColorPallet, MeshType};
use crate::{Draggable, MaterialHandles, MeshHandles};

use super::input_node::*;
use super::output_node::*;

#[derive(Component)]
pub struct Chip {
    chip_type: String,
    inputs: Vec<Entity>,
    outputs: Vec<Entity>,
}

#[derive(Component)]
pub struct NotGate;

#[derive(Component)]
pub struct AndGate;

#[derive(Bundle)]
pub struct ChipBundle {
    chip: Chip,
    sprite: MaterialMesh2dBundle<ColorMaterial>,
    draggable: Draggable,
}

#[derive(Bundle)]
pub struct NotBundle {
    chip: ChipBundle,
    not_gate: NotGate,
}

#[derive(Bundle)]
pub struct AndBundle {
    chip: ChipBundle,
    and_gate: AndGate,
}

impl NotBundle {
    pub fn spawn(
        commands: &mut Commands,
        board: Entity,
        mut z_height_manager: &mut ResMut<z_height_manager>,
        mesh_handles: &Res<MeshHandles>,
        material_handles: &Res<MaterialHandles>,
        position: Vec2,
    ) -> Entity {
        // create the input
        let input_bundle = input_node::InputNodeBundle::new(mesh_handles, material_handles);
        let output_bundle = output_node::OutputNodeBundle::new(mesh_handles, material_handles);

        let input = commands
            .spawn(input_bundle)
            .insert(Transform::from_translation(Vec3::new(
                -NOT_GATE_WIDTH / 2.0,
                0.0,
                -0.5,
            )))
            .id();

        let output = commands
            .spawn(output_bundle)
            .insert(Transform::from_translation(Vec3::new(
                NOT_GATE_WIDTH / 2.0,
                0.0,
                -0.5,
            )))
            .id();

        let chip = Chip {
            chip_type: "not".to_string(),
            inputs: vec![input],
            outputs: vec![output],
        };

        let sprite = MaterialMesh2dBundle {
            mesh: mesh_handles[MeshType::NotGate].clone(),
            material: material_handles[ColorPallet::ChipColor].clone(),
            transform: Transform::from_translation(position.extend(z_height_manager.new_entity())),
            ..default()
        };

        let chip_bundle = ChipBundle {
            chip,
            sprite,
            draggable: Draggable::default(),
        };
        let not_chip = commands.spawn((chip_bundle, NotGate)).id();
        commands.entity(not_chip).push_children(&[input, output]);
        commands.entity(board).push_children(&[not_chip]);

        not_chip
    }
}

pub fn not_gate_system(
    not_gate_query: Query<&Chip, With<NotGate>>,
    input_query: Query<(Entity, &InputNode), Changed<InputNode>>,
    mut output_query: Query<(Entity, &mut OutputNode)>,
) {
    let changed_inputs: HashMap<Entity, &InputNode> = input_query.into_iter().collect();

    for not_gate in not_gate_query.into_iter() {
        let input = not_gate.inputs[0];

        if changed_inputs.contains_key(&input) {
            for (entity, mut output) in output_query.iter_mut() {
                if entity == not_gate.outputs[0] {
                    output.value = !changed_inputs[&input].value
                }
            }
        }
    }
}
