use crate::{
    utils::{get_mouse_position, point_in_region},
    Draggable,
};
use bevy::{prelude::*, render::primitives::Aabb};

pub(crate) fn click_and_drag_camera_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    query: Query<(&mut Transform, &Draggable, &Aabb), Without<Camera>>,
    mut camera_query: Query<(&Camera, &GlobalTransform, &mut Transform, &mut Draggable)>,
    windows: Query<&Window>,
) {
    let (camera, camera_global_transform, mut camera_transform, mut camera_draggable) =
        camera_query.single_mut();

    // allows for entities with a click and drag component to be dragged. The entity with the largest z index will be prioritized.
    let mouse_position =
        if let Some(pos) = get_mouse_position(camera, camera_global_transform, windows) {
            pos
        } else {
            return;
        };

    if mouse_button_input.just_released(MouseButton::Left) {
        camera_draggable.being_dragged = false;
    }

    if camera_draggable.being_dragged {
        camera_transform.translation.x -= mouse_position.x + camera_draggable.click_offset.x;
        camera_transform.translation.y -= mouse_position.y + camera_draggable.click_offset.y;
    }

    let mut other_valid_target = false;

    for (transform, _draggable, aabb) in &query {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            // Check if the mouse is inside the mesh
            if let Some(_dif) = point_in_region(
                mouse_position,
                transform.translation.xy(),
                aabb.half_extents.xy(),
            ) {
                other_valid_target = true;
            }
        }
    }

    if other_valid_target {
        return;
    }
    if mouse_button_input.just_pressed(MouseButton::Left) {
        camera_draggable.being_dragged = true;
        camera_draggable.click_offset = -mouse_position;
    }
}
