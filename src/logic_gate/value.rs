use bevy::prelude::*;

#[derive(Component)]
pub struct Value(pub bool);

impl Value {
    pub fn default() -> Value {
        Value(false)
    }
}
