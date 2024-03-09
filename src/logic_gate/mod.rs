pub mod chips;
pub mod input_node;
pub mod output_node;
pub mod value;

use bevy::prelude::*;

use self::chips::{and_gate_system, not_gate_system};

// make a plugin to initialize the resources
pub struct ChipPlugin;

impl Plugin for ChipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (not_gate_system, and_gate_system));
    }
}
