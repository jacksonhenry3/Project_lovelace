use crate::utils::*;
use bevy::{prelude::*, render::primitives::Aabb};

#[derive(Default, Component)]
pub(crate) struct Draggable {
    pub being_dragged: bool,
    pub click_offset: Vec2,
}

pub(crate) fn click_and_drag_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &mut Draggable, &Aabb)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let (camera, camera_transform) = camera_query.single();
    // allows for entitys with a click and drag component to be dragged. The entity with the largest z index will be prioritized.
    let Some(mouse_position) = get_mouse_position(camera, camera_transform, windows) else {
        return;
    };

    let mut sorted_query = query.iter_mut().collect::<Vec<_>>();
    sorted_query.sort_by(|a, b| {
        a.0.translation
            .z
            .partial_cmp(&b.0.translation.z)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for (transform, mut draggable, aabb) in sorted_query {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            // Check if the mouse is inside the mesh
            if let Some(dif) = point_in_region(
                mouse_position,
                transform.translation.xy(),
                aabb.half_extents.xy(),
            ) {
                draggable.being_dragged = true;
                draggable.click_offset = dif;
                break;
            }
        } else if mouse_button_input.just_released(MouseButton::Left) {
            // Stop dragging
            draggable.being_dragged = false;
        }
    }

    // Update the position of the entity being dragged
    for (mut transform, draggable, _aabb) in query.iter_mut() {
        if draggable.being_dragged {
            transform.translation.x = mouse_position.x - draggable.click_offset.x;
            transform.translation.y = mouse_position.y - draggable.click_offset.y;
        }
    }
}
