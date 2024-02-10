use bevy::prelude::*;
use rand::Rng;

use super::components::*;
use super::resources::*;
use crate::components::*;

pub fn spawn_astroid_over_time(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spawn_astroid_timer: Res<AstroidSpawnTimer>,
) {
    let mut astroid_pos1 = random_astroid_position();
    let astroid_pos2 = random_astroid_position();

    if (astroid_pos1 - astroid_pos2).abs() < 50.0 {
        if astroid_pos1 + 100.0 > 110.0 {
            astroid_pos1 -= 100.0;
        } else {
            astroid_pos1 += 100.0;
        }
    }

    if spawn_astroid_timer.timer.finished() {
        let mut astroid_size: f32 = rand::thread_rng().gen_range(25..=60) as f32;

        let texture = asset_server.load("meteor.png");

        commands
            .spawn((SpriteBundle {
                texture: texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(astroid_size, astroid_size)),
                    ..default()
                },
                transform: Transform::from_xyz(astroid_pos1, 140.0, 1.0),
                ..default()
            },))
            .insert(Astroid {
                astroid_speed: 55.0,
                rotate_direction: 2.0,
                astroid_size,
            })
            .insert(PlayingComponents);

        astroid_size = rand::thread_rng().gen_range(25..=60) as f32;

        commands
            .spawn((SpriteBundle {
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(astroid_size, astroid_size)),
                    ..default()
                },
                transform: Transform::from_xyz(astroid_pos2, 140.0, 1.0),
                ..default()
            },))
            .insert(Astroid {
                astroid_speed: 70.0,
                rotate_direction: -2.0,
                astroid_size,
            })
            .insert(PlayingComponents);
    }
}

pub fn astroid_movement(mut astroids: Query<(&mut Transform, &Astroid)>, time: Res<Time>) {
    for (mut transform, astroid) in &mut astroids {
        let astroid_movement_amount = astroid.astroid_speed * time.delta_seconds();
        transform.translation.y -= astroid_movement_amount;
    }
}

pub fn despawn_astroid(mut commands: Commands, query: Query<(Entity, &Transform), With<Astroid>>) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < -120.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn tick_astroid_timer(mut astroid_spaw_timer: ResMut<AstroidSpawnTimer>, time: Res<Time>) {
    astroid_spaw_timer.timer.tick(time.delta());
}

pub fn random_astroid_position() -> f32 {
    let mut astroid_x: f32 = rand::thread_rng().gen_range(0..120) as f32;
    let change_sign: f32 = rand::thread_rng().gen_range(0..2) as f32;
    if change_sign == 1.0 {
        astroid_x *= -1.0;
    }
    astroid_x
}

pub fn rotate_astroid(time: Res<Time>, mut query: Query<(&Astroid, &mut Transform)>) {
    for (astroid, mut transform) in query.iter_mut() {
        transform.rotation *=
            Quat::from_rotation_z(astroid.rotate_direction * time.delta_seconds());
    }
}
