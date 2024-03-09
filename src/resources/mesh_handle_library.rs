use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use enum_map::{enum_map, Enum, EnumMap};
use std::ops::Index;

use crate::sizes;

#[derive(Debug, Enum, Clone, Copy)]
pub enum MeshType {
    NotGate,
    AndGate,
    Circle,
}

#[derive(Debug, Resource)]
pub struct MeshHandles {
    pub materials: EnumMap<MeshType, Mesh2dHandle>,
}

impl Index<MeshType> for MeshHandles {
    type Output = Mesh2dHandle;

    fn index(&self, index: MeshType) -> &Self::Output {
        &self.materials[index]
    }
}

pub(crate) fn initialize_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mesh_resources = enum_map! {
        MeshType::NotGate => Mesh2dHandle(meshes.add(Rectangle::new(sizes::NOT_GATE_WIDTH,sizes::NOT_GATE_HEIGHT))),
        MeshType::AndGate => Mesh2dHandle(meshes.add(Rectangle::new(sizes::NOT_GATE_WIDTH,sizes::NOT_GATE_HEIGHT*2.0))),
       MeshType::Circle => Mesh2dHandle(meshes.add(Circle { radius: sizes::INPUT_NODE_RADIUS})),
    };

    commands.insert_resource(MeshHandles {
        materials: mesh_resources,
    });
}

pub fn draw_line(
    commands: &mut Commands,
    start: Vec2,
    end: Vec2,
    thickness: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    material_handle: Handle<ColorMaterial>,
    parent: Entity,
) {
    let line_length = start.distance(end);
    let midpoint = (start + end) / 2.0;
    let rotation = (end - start).angle_between(Vec2::X);

    let line = commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(line_length, thickness))),
            material: material_handle.clone(),
            transform: Transform {
                translation: Vec3::new(midpoint.x, midpoint.y, 0.0),
                rotation: Quat::from_rotation_z(-rotation),
                scale: Vec3::ONE,
            },
            ..Default::default()
        })
        .id();

    // draw a circle at either end of the line
    let circle_radius = thickness * 0.5;

    let circle = commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle {
                radius: circle_radius,
            })),
            material: material_handle.clone(),
            transform: Transform {
                translation: Vec3::new(start.x, start.y, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            ..Default::default()
        })
        .id();
    let mut parent = commands.entity(parent);
    parent.push_children(&[line]);
    parent.push_children(&[circle]);
}

#[derive(Component)]
struct Path;

pub fn draw_path(
    commands: &mut Commands,
    path: &[Vec2],
    thickness: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    material_handle: Handle<ColorMaterial>,
) -> Entity {
    let path_entity = commands.spawn((Path, SpatialBundle::default())).id();

    for i in 0..path.len() - 1 {
        let start = path[i];
        let end = path[i + 1];
        draw_line(
            commands,
            start,
            end,
            thickness,
            meshes,
            material_handle.clone(),
            path_entity,
        );
    }

    // add a circle to the end of the path
    let end_point = path.last().cloned().unwrap();
    let end_cap = commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle {
                radius: thickness * 0.5,
            })),
            material: material_handle.clone(),
            transform: Transform {
                translation: Vec3::new(end_point.x, end_point.y, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            ..Default::default()
        })
        .id();

    commands.entity(path_entity).push_children(&[end_cap]);

    path_entity
}
