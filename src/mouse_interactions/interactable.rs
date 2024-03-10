use crate::*;

#[derive(Default, Component)]
pub struct Interactable {
    pub mouse_event: Option<Mouse>,
}

pub fn hover_system(
    mut query: Query<(&mut Interactable, &GlobalTransform, &Aabb)>,
    ui_query: Query<&Interaction>,
    mouse: Res<Mouse>,
) {
    // don't do anything if the mouse is on the UI
    for interaction in &ui_query {
        if *interaction != Interaction::None {
            return;
        }
    }

    // Each entity that the mouse is over gets a copy of the current state of the mouse, allowing them to detect all mouse button inputs, and know its position.
    for (mut interactable, _global_transform, _aabb) in query.iter_mut() {
        interactable.mouse_event = None;
    }

    // sort the query by z height so we can only hover the top one
    let mut sorted_query = query.iter_mut().collect::<Vec<_>>();

    sorted_query.sort_by(|a, b| {
        b.1.translation()
            .z
            .partial_cmp(&a.1.translation().z)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // The highest entity that the mouse is over gets a copy of the current state of the mouse, allowing them to detect all mouse button inputs, and know its position.
    for (mut interactable, global_transform, aabb) in sorted_query {
        if let Some(_test) = point_in_region(
            mouse.position,
            global_transform.translation().xy(),
            aabb.half_extents.xy(),
        ) {
            interactable.mouse_event = Some(mouse.clone());
            break;
        }
    }
}
