use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_prototype_lyon::prelude::*;
use enum_map::{enum_map, Enum, EnumMap};
use std::ops::Index;

#[derive(Debug, Enum, Clone, Copy)]
pub enum MeshType {
    Rectangle,
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
       MeshType::Rectangle => Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
       MeshType::Circle => Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
    };

    commands.insert_resource(MeshHandles {
        materials: mesh_resources,
    });
}

pub(crate) fn line(
    points: Vec<Vec2>,
) -> (
    bevy_prototype_lyon::entity::ShapeBundle,
    bevy_prototype_lyon::draw::Stroke,
) {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::new(0., 0.));

    for window in points.windows(4) {
        if let [start, control1, control2, end] = window {
            let control1 = calculate_control_point(*start, *control1, *control2, 0.9);
            let control2 = calculate_control_point(*control2, control1, *end, 0.9);
            path_builder.quadratic_bezier_to(control1, control2);
        }
    }
    let path = path_builder.build();

    (
        ShapeBundle {
            path,
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0., 75., 0.),
                ..default()
            },
            ..default()
        },
        Stroke::new(Color::RED, 1.0),
    )
}

fn calculate_control_point(start: Vec2, control: Vec2, end: Vec2, smoothness: f32) -> Vec2 {
    let vec1 = control - start;
    let vec2 = end - control;

    let control_point = control + vec1.lerp(vec2, smoothness);
    control_point
}
