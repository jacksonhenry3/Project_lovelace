use bevy::prelude::*;

pub(crate) fn get_mouse_position(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    windows: Query<&Window>,
) -> Option<Vec2> {
    let Some(cursor_position) = windows.single().cursor_position() else {
        return None;
    };

    // Calculate a world position based on the cursor's position.
    camera.viewport_to_world_2d(camera_transform, cursor_position)
}

pub(crate) fn point_in_region(point: Vec2, center: Vec2, extents: Vec2) -> Option<Vec2> {
    let min_x = center.x - extents.x;
    let max_x = center.x + extents.x;
    let min_y = center.y - extents.y;
    let max_y = center.y + extents.y;
    if point.x > min_x && point.x < max_x && point.y > min_y && point.y < max_y {
        Some(point - center.xy())
    } else {
        None
    }
}
