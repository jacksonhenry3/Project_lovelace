use crate::*;

#[derive(Default, Component)]
pub struct Draggable {
    offset: Option<Vec2>,
}

#[derive(Bundle, Default)]
pub struct DraggableBundle {
    pub draggable: Draggable,
    pub interactable: Interactable,
}

pub fn initiate_drag_system(
    mut query: Query<(&mut Transform, &mut Draggable, &Interactable)>,
    mut z_height_manager: ResMut<z_height_manager>,
) {
    for (mut transform, mut draggable, interactable) in query.iter_mut() {
        if let Some(mouse) = &interactable.mouse_event {
            if mouse.input.just_pressed(MouseButton::Left) {
                let drag_offset = transform.translation.xy() - mouse.position;
                draggable.offset = Some(drag_offset);
                transform.translation.z = z_height_manager.selected(transform.translation.z);
            }
            if mouse.input.just_released(MouseButton::Left) {
                draggable.offset = None;
            }
        }
    }
}

pub fn drag_system(mut query: Query<(&mut Transform, &Draggable)>, mouse: Res<Mouse>) {
    let mut draggable_entities: Vec<_> = query.iter_mut().collect();

    // Sort entities by transform.translation.z
    draggable_entities.sort_by(|a, b| a.0.translation.z.partial_cmp(&b.0.translation.z).unwrap());

    for (mut transform, draggable) in draggable_entities {
        if let Some(offset) = draggable.offset {
            transform.translation.x = mouse.position.x + offset.x;
            transform.translation.y = mouse.position.y + offset.y;
            break;
        }
    }
}
