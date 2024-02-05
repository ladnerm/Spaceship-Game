use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use rand::Rng;

pub const FIREBALL_SPAWN_TIME: f32 = 1.2;
pub const COIN_SPAWN_TIME: f32 = 3.0;
pub const COIN_SIZE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Space Game".into(),
                    resolution: (640.0, 480.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .build()
        )
        .init_resource::<CoinSpawnTimer>()
        .init_resource::<FireballSpawnTimer>()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .add_systems(Update, (spawn_fireball_over_time, tick_fireball_timer))
        .add_systems(Update, (tick_coin_timer, spawn_coin_over_time))
        .add_systems(Update, (fireball_movement, rotate_fireball))
        .add_systems(Update, (despawn_fireball, check_collision))
                .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player { speed: 80.0},
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
)  {
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

fn fireball_movement(
    mut fireballs: Query<(&mut Transform, &Fireball)>,
    time: Res<Time>
) {
    for (mut transform, fireball) in &mut fireballs {
        let fireball_movement_amount = fireball.fireball_speed * time.delta_seconds();
        transform.translation.y -= fireball_movement_amount;
    }
}

fn despawn_fireball(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Fireball>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < -90.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn check_collision(
    query_player: Query<(Entity, &Transform), With<Player>>,
    query_fireball: Query<(&Transform, &Fireball), With<Fireball>>,
    query_coin: Query<(Entity, &Transform), With<Coin>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    for (player_entity, player_transform) in query_player.iter() {
        for (fireball_transform, fireball) in query_fireball.iter() {
            let player_position = player_transform;
            let fireball_position = fireball_transform;
            if meteor_is_colliding(player_position, fireball_position, (fireball.fireball_size) as f64) {
                commands.entity(player_entity).despawn();
                commands.spawn(AudioBundle {
                    source: asset_server.load("crash.ogg"),
                    ..default()
                });
            }
        }
        for (coin_entity, coin_transform) in query_coin.iter() {
            let player_position = player_transform;
            let coin_position = coin_transform;
            if coin_is_colliding(player_position, coin_position, COIN_SIZE as f64) {
                commands.entity(coin_entity).despawn();
                commands.spawn(AudioBundle {
                    source: asset_server.load("coinsfx.ogg"),
                    ..default()
                });
            }
        }
    }
}

fn meteor_is_colliding(
    position1: &Transform, 
    position2: &Transform,
    thing_size: f64
) -> bool{
    let distance_threshold = thing_size-(thing_size*0.44);
    let p1x = position1.translation.x;
    let p1y = position1.translation.y;
    let p2x = position2.translation.x;
    let p2y = position2.translation.y;

    (((p1x-p2x)*(p1x-p2x)+(p1y-p2y)*(p1y-p2y)) as f64).sqrt() < distance_threshold
}

fn coin_is_colliding(
    position1: &Transform, 
    position2: &Transform,
    thing_size: f64
) -> bool{
    let distance_threshold = thing_size;
    let p1x = position1.translation.x;
    let p1y = position1.translation.y;
    let p2x = position2.translation.x;
    let p2y = position2.translation.y;

    (((p1x-p2x)*(p1x-p2x)+(p1y-p2y)*(p1y-p2y)) as f64).sqrt() < distance_threshold
}

pub fn tick_fireball_timer(mut fireball_spaw_timer: ResMut<FireballSpawnTimer>, time: Res<Time>) {
    fireball_spaw_timer.timer.tick(time.delta());
}

pub fn random_fireball_position() -> f32 {
    let mut fireball_x: f32 = rand::thread_rng().gen_range(0..120) as f32;
    let change_sign: f32 = rand::thread_rng().gen_range(0..2) as f32;
    if change_sign == 1.0 {
        fireball_x *= -1.0;
    }
    fireball_x
}

pub fn spawn_fireball_over_time(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spawn_fireball_timer: Res<FireballSpawnTimer>,
) {
    let mut fireball_pos1 = random_fireball_position();
    let fireball_pos2 = random_fireball_position();

    if (fireball_pos1-fireball_pos2).abs() < 50.0 {
        if fireball_pos1 + 100.0 > 110.0 {
            fireball_pos1 -= 100.0;
        } else {
            fireball_pos1 += 100.0;
        }
    }


    if spawn_fireball_timer.timer.finished() {

        let mut fireball_size: f32 = rand::thread_rng().gen_range(25..=60) as f32;

        let texture = asset_server.load("meteor.png");
     
        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(fireball_size, fireball_size)),
                    ..default()
                },
                transform: Transform::from_xyz(fireball_pos1, 140.0, 0.0),
                ..default()
            },
        ))
        .insert(Fireball { 
            fireball_speed: 55.0, 
            rotate_direction: 2.0, 
            fireball_size 
        });

        fireball_size = rand::thread_rng().gen_range(25..=60) as f32;

        commands.spawn((
            SpriteBundle {
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(fireball_size, fireball_size)),
                    ..default()
                },
                transform: Transform::from_xyz(fireball_pos2, 140.0, 0.0),
                ..default()
            },
        ))
        .insert(Fireball { fireball_speed: 70.0, 
            rotate_direction: -2.0,
            fireball_size
        });
    }
}

fn rotate_fireball(time: Res<Time>, mut query: Query<(&Fireball, &mut Transform)>) {

    for (fireball, mut transform) in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_z(fireball.rotate_direction * time.delta_seconds());
    }
}

fn tick_coin_timer(mut coin_spaw_timer: ResMut<CoinSpawnTimer>, time: Res<Time>) {
    coin_spaw_timer.timer.tick(time.delta());
}

fn spawn_coin_over_time(
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

        commands.spawn(
            SpriteBundle {
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(COIN_SIZE, COIN_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(random_coin_x, random_coin_y, 0.0),
                ..default()
            }
        )
        .insert( Coin {} );
    }
}

#[derive(Component)]
struct Player {
    pub speed: f32,
}

#[derive(Component)]
struct Fireball {
    pub fireball_speed: f32,
    pub rotate_direction: f32,
    pub fireball_size: f32,
}

#[derive(Component)]
pub struct Coin {

}

#[derive(Resource)]
pub struct FireballSpawnTimer {
    pub timer: Timer,
}

impl Default for FireballSpawnTimer {
    fn default() -> FireballSpawnTimer {
        FireballSpawnTimer { 
            timer: Timer::from_seconds(FIREBALL_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

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
