use bevy::prelude::*;

pub const COIN_SPAWN_TIME: f32 = 3.0;

#[derive(Resource)]
pub struct CoinSpawnTimer {
    pub timer: Timer,
}

impl Default for CoinSpawnTimer {
    fn default() -> CoinSpawnTimer {
        CoinSpawnTimer { 
            timer: Timer::from_seconds(COIN_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}