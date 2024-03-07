mod click_and_drag_world;
mod zoom_world;

use bevy::prelude::*;

pub struct WorldInteractionPlugin;

impl Plugin for WorldInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                zoom_world::camera_zoom_system,
                click_and_drag_world::click_and_drag_camera_system,
            ),
        );
    }
}
