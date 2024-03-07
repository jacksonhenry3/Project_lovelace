use bevy::math::vec2;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_prototype_lyon::prelude::*;
mod resources;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use resources::material_handle_library::{ColorPallet, MaterialHandles};
use resources::mesh_handle_library::{self, MeshHandles, MeshType};
mod click_and_drag;
use click_and_drag::*;
mod utils;
use rand::Rng;
use world_control::WorldInteractionPlugin;

mod world_control;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            resources::ResourcesPlugin,
            WorldInteractionPlugin,
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_cursor, click_and_drag_system))
        .add_plugins(ShapePlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mesh_handles: Res<MeshHandles>,
    material_handles: Res<MaterialHandles>,
) {
    commands.spawn((Camera2dBundle::default(), Draggable::default()));

    let mesh_handle = mesh_handles[MeshType::Rectangle].clone();
    let material_handle = material_handles[ColorPallet::Red].clone();

    for i in 0..1 {
        let x_position = i as f32 * 10.0 - 100.0; // Adjust this to set the positions

        commands.spawn((
            Draggable {
                being_dragged: false,
                click_offset: Vec2::ZERO,
            },
            MaterialMesh2dBundle {
                mesh: mesh_handle.clone(),
                material: material_handle.clone(),
                transform: Transform::from_xyz(x_position, 0.0, 0.0),
                ..Default::default()
            },
        ));
    }

    let mut rng = rand::thread_rng();

    // let amplitude = 50.0;
    // let frequency = 10.0;
    let points: Vec<Vec2> = (0..100)
        .map(|_i| {
            let x = rng.gen_range(-100.0..100.0);
            let y = rng.gen_range(-100.0..100.0);
            vec2(x, y)
        })
        .collect();

    commands.spawn(mesh_handle_library::line(points));

    // spawn 100 random
}

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., Color::WHITE);
}
