pub mod chips;
pub mod input_node;
pub mod output_node;

use bevy::prelude::*;

use self::chips::not_gate_system;

// make a plugin to initialize the resources
pub struct ChipPlugin;

impl Plugin for ChipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, not_gate_system);
    }
}
