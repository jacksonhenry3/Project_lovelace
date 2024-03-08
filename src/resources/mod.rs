use bevy::prelude::*;

pub mod colors;
pub mod game_state;
pub mod material_handle_library;
pub mod mesh_handle_library;
pub mod sizes;
pub mod z_height;

pub use colors::*;
pub use material_handle_library::*;
pub use mesh_handle_library::*;
pub use z_height::*;

// make a plugin to initialize the resources
pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<colors::Colors>()
            .init_resource::<MaterialHandles>()
            .init_resource::<z_height_manager>()
            .add_systems(PreStartup, (mesh_handle_library::initialize_meshes,))
            .add_systems(Update, color_system)
            .init_state::<game_state::GameState>();
    }
}
