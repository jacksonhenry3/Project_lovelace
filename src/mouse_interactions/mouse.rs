use crate::*;

#[derive(Resource, Clone)]
pub struct Mouse {
    pub position: Vec2,
    pub input: ButtonInput<MouseButton>,
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            position: Vec2::INFINITY,
            input: ButtonInput::default(),
        }
    }
}

pub(crate) fn mouse_update_system(
    mut mouse: ResMut<Mouse>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let (camera, camera_transform) = camera_query.single();
    let Some(mouse_position) = get_mouse_position(camera, camera_transform, windows) else {
        return;
    };

    mouse.position = mouse_position;
    mouse.input = mouse_button_input.clone();
}
