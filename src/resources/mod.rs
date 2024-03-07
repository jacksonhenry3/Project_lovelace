use bevy::prelude::*;

pub mod material_handle_library;
pub mod mesh_handle_library;

// make a plugin to initialize the resources
pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreStartup,
            (
                material_handle_library::initialize_colors,
                mesh_handle_library::initialize_meshes,
            ),
        );
    }
}
