use bevy::prelude::*;

pub mod mouse;
pub use mouse::*;

pub mod interactable;
pub use interactable::*;

pub mod drag;
pub use drag::*;

pub struct MouseInteractionPlugin;

impl Plugin for MouseInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                hover_system,
                initiate_drag_system,
                mouse_update_system,
                drag_system,
            ),
        )
        .init_resource::<Mouse>();
    }
}
