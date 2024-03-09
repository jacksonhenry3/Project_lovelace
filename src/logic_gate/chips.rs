use bevy::sprite;
use bevy::utils::HashMap;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::logic_gate::{input_node, output_node};
use crate::sizes::{NOT_GATE_HEIGHT, NOT_GATE_WIDTH};
use crate::{sizes, z_height_manager, ColorPallet, MeshType};
use crate::{Draggable, MaterialHandles, MeshHandles};

use super::input_node::*;
use super::output_node::*;
use super::value::Value;

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
    input_query: Query<(&Value), (With<InputNode>, Changed<Value>)>,
    mut output_query: Query<&mut Value, (With<OutputNode>, Without<InputNode>)>,
) {
    for not_gate in not_gate_query.into_iter() {
        let input = not_gate.inputs[0];

        if let Ok(input_value) = input_query.get(input) {
            output_query.get_mut(not_gate.outputs[0]).unwrap().0 = !input_value.0;
        }
    }
}

impl AndBundle {
    pub fn spawn(
        commands: &mut Commands,
        board: Entity,
        mut z_height_manager: &mut ResMut<z_height_manager>,
        mesh_handles: &Res<MeshHandles>,
        material_handles: &Res<MaterialHandles>,
        position: Vec2,
    ) -> Entity {
        // create the inputs
        let input_bundle1 = input_node::InputNodeBundle::new(mesh_handles, material_handles);
        let input_bundle2 = input_node::InputNodeBundle::new(mesh_handles, material_handles);
        let output_bundle = output_node::OutputNodeBundle::new(mesh_handles, material_handles);

        let input1 = commands
            .spawn(input_bundle1)
            .insert(Transform::from_translation(Vec3::new(
                -NOT_GATE_WIDTH / 2.,
                NOT_GATE_HEIGHT / 2.,
                -0.5,
            )))
            .id();

        let input2 = commands
            .spawn(input_bundle2)
            .insert(Transform::from_translation(Vec3::new(
                -NOT_GATE_WIDTH / 2.,
                -NOT_GATE_HEIGHT / 2.,
                -0.5,
            )))
            .id();

        let output = commands
            .spawn(output_bundle)
            .insert(Transform::from_translation(Vec3::new(
                NOT_GATE_WIDTH / 2.,
                0.0,
                -0.5,
            )))
            .id();

        let chip = Chip {
            chip_type: "and".to_string(),
            inputs: vec![input1, input2],
            outputs: vec![output],
        };

        let sprite = MaterialMesh2dBundle {
            mesh: mesh_handles[MeshType::AndGate].clone(),
            material: material_handles[ColorPallet::ChipColor].clone(),
            transform: Transform::from_translation(position.extend(z_height_manager.new_entity())),
            ..default()
        };

        let chip_bundle = ChipBundle {
            chip,
            sprite,
            draggable: Draggable::default(),
        };
        let and_chip = commands.spawn((chip_bundle, AndGate)).id();
        commands
            .entity(and_chip)
            .push_children(&[input1, input2, output]);
        commands.entity(board).push_children(&[and_chip]);

        and_chip
    }
}

pub fn and_gate_system(
    and_gate_query: Query<&Chip, With<AndGate>>,
    change_input_query: Query<Entity, (With<InputNode>, Changed<Value>)>,
    input_query: Query<(&Value), With<InputNode>>,
    mut output_query: Query<&mut Value, (With<OutputNode>, Without<InputNode>)>,
) {
    for and_gate in and_gate_query.iter() {
        let input1 = and_gate.inputs[0];
        let input2 = and_gate.inputs[1];

        let input_1_change = change_input_query.get(input1);
        let input_2_change = change_input_query.get(input2);

        if input_1_change.is_ok() || input_2_change.is_ok() {
            let input1_value = input_query.get(input1).ok().unwrap().0;
            let input2_value = input_query.get(input2).ok().unwrap().0;

            let mut output = output_query.get_mut(and_gate.outputs[0]).unwrap();
            output.0 = input1_value && input2_value;
        }
    }
}
