mod zoom_world;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::*;

pub struct WorldInteractionPlugin;

impl Plugin for WorldInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                zoom_world::camera_zoom_system,
                // click_and_drag_world::click_and_drag_camera_system,
            ),
        );
    }
}

#[derive(Component)]
pub struct Board;

pub fn make_board(
    meshes: &mut ResMut<Assets<Mesh>>,
    material_handles: &Res<MaterialHandles>,
) -> (
    DraggableBundle,
    MaterialMesh2dBundle<bevy::prelude::ColorMaterial>,
    Board,
) {
    let mesh_handle = Mesh2dHandle(meshes.add(Rectangle::new(
        sizes::BOARD_EDGE_LENGTH,
        sizes::BOARD_EDGE_LENGTH,
    )));
    let material_handle = material_handles[ColorPallet::BoardColor].clone();

    (
        DraggableBundle::default(),
        MaterialMesh2dBundle {
            mesh: mesh_handle.clone(),
            material: material_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, -2.0),
            ..Default::default()
        },
        Board,
    )
}
