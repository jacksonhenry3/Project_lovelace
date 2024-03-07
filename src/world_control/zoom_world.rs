use crate::utils::get_mouse_position;
use bevy::{input::mouse::MouseWheel, prelude::*};
pub(crate) fn camera_zoom_system(
    mut event_reader: EventReader<MouseWheel>,
    mut query: Query<(&Camera, &mut Transform, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let zoom_factor: f32 = -0.1; // Adjust as needed
    let mut zoom_delta = 0.0;

    // Listen for mouse wheel events
    for event in event_reader.read() {
        zoom_delta += event.y;
    }

    let (camera, mut camera_transform, camera_global_transform) = query.single_mut();

    // Adjust the scale and translation of each camera
    // for (_camera, mut transform, global_transform) in query.iter_mut() {
    let scale = camera_transform.scale.x + zoom_delta * zoom_factor;
    let scale = Vec3::splat(scale.clamp(0.1, 10.0));
    let mouse_world_position = get_mouse_position(camera, camera_global_transform, windows)
        .unwrap_or(Vec2::ZERO)
        .extend(0.0);
    let translation = camera_transform.translation
        + (mouse_world_position - camera_transform.translation)
            * (1.0 - scale / camera_transform.scale.x);
    camera_transform.scale = scale; // Clamp the scale to prevent it from getting too small or too large
    camera_transform.translation = translation;
    // }
}
