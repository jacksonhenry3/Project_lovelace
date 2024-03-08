use crate::{utils::*, z_height_manager};
use bevy::{prelude::*, render::primitives::Aabb};

#[derive(Default, Component)]
pub(crate) struct Draggable {
    pub being_dragged: bool,
    pub click_offset: Vec2,
}

pub(crate) fn click_and_drag_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &GlobalTransform, &mut Draggable, &Aabb)>,
    mut z_height_manager: ResMut<z_height_manager>,
    ui_query: Query<&Interaction>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    for interaction in &ui_query {
        if *interaction != Interaction::None {
            return;
        }
    }
    let (camera, camera_transform) = camera_query.single();
    // allows for entitys with a click and drag component to be dragged. The entity with the largest z index will be prioritized.
    let Some(mouse_position) = get_mouse_position(camera, camera_transform, windows) else {
        return;
    };

    let mut sorted_query = query.iter_mut().collect::<Vec<_>>();
    sorted_query.sort_by(|a, b| {
        b.1.translation()
            .z
            .partial_cmp(&a.1.translation().z)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for (mut transform, global_transform, mut draggable, aabb) in sorted_query {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            // Check if the mouse is inside the mesh
            if let Some(dif) = point_in_region(
                mouse_position,
                global_transform.translation().xy(),
                aabb.half_extents.xy(),
            ) {
                // update the z height of whatever was selected so that it is on top
                transform.translation.z = z_height_manager.selected(transform.translation.z);

                draggable.being_dragged = true;
                draggable.click_offset =
                    dif + global_transform.translation().xy() - transform.translation.xy();
                break;
            }
        } else if mouse_button_input.just_released(MouseButton::Left) {
            // Stop dragging
            draggable.being_dragged = false;
        }
    }

    // Update the position of the entity being dragged
    for (mut transform, _global_transform, draggable, _aabb) in query.iter_mut() {
        if draggable.being_dragged {
            transform.translation.x = mouse_position.x - draggable.click_offset.x;
            transform.translation.y = mouse_position.y - draggable.click_offset.y;
        }
    }
}
