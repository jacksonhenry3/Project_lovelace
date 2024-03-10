use bevy::prelude::*;

#[derive(Debug, Resource, Default)]
pub struct z_height_manager {
    pub heights: Vec<f32>,
}

impl z_height_manager {
    pub fn new_entity(&mut self) -> f32 {
        let mut height = 2.0;
        while self.heights.contains(&height) {
            height += 1.0;
        }

        self.heights.push(height);
        height
    }

    pub fn selected(&mut self, current_height: f32) -> f32 {
        if current_height == 0.0 {
            return 0.0;
        }
        // remove current_height from self.heights
        // this assumes there is only ever one object with a given height, which is the goal.
        self.heights.retain(|&h| h != current_height);
        let new_height = self.heights.iter().cloned().fold(0.0, f32::max) + 1.0;
        self.heights.push(new_height);
        new_height
    }
}
