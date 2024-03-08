use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct z_height_manager {
    pub heights: Vec<f32>,
}

impl Default for z_height_manager {
    fn default() -> Self {
        z_height_manager { heights: vec![] }
    }
}

impl z_height_manager {
    pub fn new_entity(&mut self) -> f32 {
        let mut height = 2.0;
        while self.heights.contains(&height) {
            height += 1.0;
        }

        self.heights.push(height);
        return height;
    }

    pub fn selected(&mut self, current_height: f32) -> f32 {
        if current_height == 0.0 {
            return 0.0;
        }
        // remove current_height from self.heights
        // this assumes there is only ever one object with a given height, which is the goal.
        self.heights.retain(|&h| h != current_height);
        let new_height = self
            .heights
            .iter()
            .cloned()
            .fold(f32::NEG_INFINITY, f32::max)
            + 1.0;
        self.heights.push(new_height);
        new_height
    }
}
