use bevy::prelude::*;

pub mod chips;
pub mod input_node;
pub mod output_node;
pub mod value;

pub use chips::*;
pub use input_node::*;
pub use output_node::*;
pub use value::*;

pub struct ChipPlugin;

impl Plugin for ChipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (not_gate_system, and_gate_system));
    }
}
