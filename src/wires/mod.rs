use bevy::{prelude::*, render::primitives::Aabb};

use crate::{
    logic_gate::{input_node::InputNode, output_node::OutputNode},
    utils::{get_mouse_position, point_in_region},
};

#[derive(Default, Component)]
pub(crate) struct Connectable {
    pub being_connected: bool,
}

pub(crate) fn connection_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut inputs_query: Query<(Entity, &GlobalTransform, &Aabb, &mut Connectable), With<InputNode>>,
    mut outputs_query: Query<
        (
            Entity,
            &GlobalTransform,
            &Aabb,
            &mut Connectable,
            &mut OutputNode,
        ),
        Without<InputNode>,
    >,

    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let (camera, camera_transform) = camera_query.single();
    // allows for entitys with a click and drag component to be dragged. The entity with the largest z index will be prioritized.
    let Some(mouse_position) = get_mouse_position(camera, camera_transform, windows) else {
        return;
    };

    let being_connected = &outputs_query
        .iter()
        .filter(|(_, _, _, c, _)| c.being_connected)
        .map(|(e, _, _, _, _)| e)
        .collect::<Vec<_>>();

    if mouse_button_input.just_pressed(MouseButton::Left) {
        for (_, global_transform, aabb, mut connectable, mut output) in &mut outputs_query {
            // Check if the mouse is inside the mesh
            if let Some(dif) = point_in_region(
                mouse_position,
                global_transform.translation().xy(),
                aabb.half_extents.xy(),
            ) {
                connectable.being_connected = true;
                println!("connecting ");
                // output.endpoints.push(mouseobject)
                break;
            }
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        for (input_id, global_transform, aabb, mut connectable) in &inputs_query {
            // Check if the mouse is inside the mesh
            if let Some(dif) = point_in_region(
                mouse_position,
                global_transform.translation().xy(),
                aabb.half_extents.xy(),
            ) {
                for output in being_connected {
                    if let Ok((_, _, c, _, mut output_node)) = outputs_query.get_mut(*output) {
                        output_node.endpoints.push(input_id);
                        println!("pushed");
                    }
                }
            }
        }
    }
}
