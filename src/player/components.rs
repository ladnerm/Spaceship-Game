use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub score: i8,
}

#[derive(Component)]
pub struct HighScoreText;
