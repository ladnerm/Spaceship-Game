use bevy::prelude::*;

pub const ASTROID_SPAWN_TIME: f32 = 1.2;

#[derive(Resource)]
pub struct AstroidSpawnTimer {
    pub timer: Timer,
}

impl Default for AstroidSpawnTimer {
    fn default() -> AstroidSpawnTimer {
        AstroidSpawnTimer { 
            timer: Timer::from_seconds(ASTROID_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}