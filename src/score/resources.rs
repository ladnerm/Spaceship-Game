use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct HighScore {
    pub score: i8,
}