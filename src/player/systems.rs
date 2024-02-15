use bevy::prelude::*;

use super::components::*;

use crate::astroid::components::Astroid;
use crate::coin::components::Coin;
use crate::coin::systems::COIN_SIZE;
use crate::components::*;
use crate::events::GameState;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("spaceship.png");

    commands
        .spawn((
            SpriteBundle {
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(20.0, 19.0)),
                    ..default()
                },
                ..default()
            },
            Player {
                speed: 80.0,
                score: 0,
            },
        ))
        .insert(PlayingComponents);
}

pub fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();
        if transform.translation.x.abs() < 120.0 && transform.translation.y.abs() < 90.0 {
            if input.pressed(KeyCode::W) {
                transform.translation.y += movement_amount;
            }
            if input.pressed(KeyCode::S) {
                transform.translation.y -= movement_amount;
            }
            if input.pressed(KeyCode::A) {
                transform.translation.x -= movement_amount;
            }
            if input.pressed(KeyCode::D) {
                transform.translation.x += movement_amount;
            }
        } else if transform.translation.x > 120.0 {
            transform.translation.x -= movement_amount;
        } else if transform.translation.y > 90.0 {
            transform.translation.y -= movement_amount;
        } else if transform.translation.x < -120.0 {
            transform.translation.x += movement_amount;
        } else if transform.translation.y < -90.0 {
            transform.translation.y += movement_amount;
        }
    }
}

pub fn check_collision(
    query_player: Query<&Transform, With<Player>>,
    mut query_player_score: Query<&mut Player>,
    query_astroid: Query<(&Transform, &Astroid), With<Astroid>>,
    query_coin: Query<(Entity, &Transform), With<Coin>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<GameState>>,
    playing_items_query: Query<Entity, With<PlayingComponents>>,
    music_query: Query<Entity, With<Music>>,
    high_score_query: Query<Entity, With<HighScoreText>>,
) {
    for player_transform in query_player.iter() {
        for (astroid_transform, astroid) in query_astroid.iter() {
            let player_position = player_transform;
            let astroid_position = astroid_transform;
            if player_colliding_with_meteor(
                player_position,
                astroid_position,
                (astroid.astroid_size) as f64,
            ) {
                commands.spawn(AudioBundle {
                    source: asset_server.load("crash.ogg"),
                    ..default()
                });
                for high_score_text in high_score_query.iter() {
                    commands.entity(high_score_text).despawn();
                }
                for playing_items in playing_items_query.iter() {
                    commands.entity(playing_items).despawn();
                }
                for music in music_query.iter() {
                    commands.entity(music).despawn();
                }
                game_state.set(GameState::Menu);
            }
        }
        for (coin_entity, coin_transform) in query_coin.iter() {
            let player_position = player_transform;
            let coin_position = coin_transform;
            if player_colliding_with_coin(player_position, coin_position, COIN_SIZE as f64) {
                commands.entity(coin_entity).despawn();
                commands.spawn(AudioBundle {
                    source: asset_server.load("coinsfx.ogg"),
                    ..default()
                });

                for mut player_score in &mut query_player_score {
                    player_score.score += 1;
                }
            }
        }
    }
}

pub fn player_colliding_with_meteor(
    position1: &Transform,
    position2: &Transform,
    thing_size: f64,
) -> bool {
    let distance_threshold = thing_size - (thing_size * 0.44);
    let p1x = position1.translation.x;
    let p1y = position1.translation.y;
    let p2x = position2.translation.x;
    let p2y = position2.translation.y;

    (((p1x - p2x) * (p1x - p2x) + (p1y - p2y) * (p1y - p2y)) as f64).sqrt() < distance_threshold
}

pub fn player_colliding_with_coin(
    position1: &Transform,
    position2: &Transform,
    thing_size: f64,
) -> bool {
    let distance_threshold = thing_size;
    let p1x = position1.translation.x;
    let p1y = position1.translation.y;
    let p2x = position2.translation.x;
    let p2y = position2.translation.y;

    (((p1x - p2x) * (p1x - p2x) + (p1y - p2y) * (p1y - p2y)) as f64).sqrt() < distance_threshold
}
