use bevy::prelude::*;

#[derive(Component)]
pub struct Astroid {
    pub astroid_speed: f32,
    pub rotate_direction: f32,
    pub astroid_size: f32,
}