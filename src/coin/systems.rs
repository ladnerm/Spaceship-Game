use bevy::prelude::*;
use rand::Rng;

use super::components::*;
use super::resources::*;
use crate::components::*;

pub const COIN_SIZE: f32 = 10.0;

pub fn tick_coin_timer(mut coin_spaw_timer: ResMut<CoinSpawnTimer>, time: Res<Time>) {
    coin_spaw_timer.timer.tick(time.delta());
}

pub fn spawn_coin_over_time(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spawn_coin_timer: Res<CoinSpawnTimer>,
) {
    if spawn_coin_timer.timer.finished() {
        let mut random_coin_x: f32 = rand::thread_rng().gen_range(0..=125) as f32;
        let mut random_coin_y: f32 = rand::thread_rng().gen_range(0..=60) as f32;
        let mut change_sign: f32 = rand::thread_rng().gen_range(0..2) as f32;

        if change_sign == 0.0 {
            random_coin_x *= -1.0;
        }

        change_sign = rand::thread_rng().gen_range(0..2) as f32;

        if change_sign == 0.0 {
            random_coin_y *= -1.0;
        }

        let texture = asset_server.load("coin.png");

        commands
            .spawn(SpriteBundle {
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(COIN_SIZE, COIN_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(random_coin_x, random_coin_y, 0.0),
                ..default()
            })
            .insert(Coin)
            .insert(PlayingComponents);
    }
}
