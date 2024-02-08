use bevy::prelude::*;

pub const FIREBALL_SPAWN_TIME: f32 = 1.2;
pub const COIN_SPAWN_TIME: f32 = 3.0;
pub const COIN_SIZE: f32 = 10.0;


use bevy::render::camera::ScalingMode;
use rand::Rng;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    StartMenu,
    Playing,
}

#[derive(Component)]
struct MenuComponents();

#[derive(Component)]
struct PlayingComponents();

#[derive(Component)]
struct Music;

fn play_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("WMusic.ogg"),
        ..default()
    })
    .insert(Music);
}

fn setup_menu(mut commands: Commands) {
    
    let mut start_camera = Camera2dBundle::default();

    start_camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(start_camera)
    .insert(MenuComponents());

    commands.spawn(TextBundle::from_section(
        "Press Space to Begin",
        TextStyle {
            font_size: 30.0,
            ..default()
        },

    )
    .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(100.0),
            right: Val::Px(100.0),
            ..default()
        })
    )
    .insert(MenuComponents());
}

fn state_transition(
    mut game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    menu_items_query: Query<Entity, With<MenuComponents>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Playing);
        for entity in menu_items_query.iter() {
            commands.entity(entity).despawn();
        }
    }
    
}

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
        .add_state::<GameState>()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(OnEnter(GameState::StartMenu), (setup_menu, play_music))
        .add_systems(Update, state_transition)
        .add_systems(OnEnter(GameState::Playing), game_setup)
        .add_systems(Update, (
            character_movement,
            spawn_fireball_over_time,
            tick_fireball_timer,
            tick_coin_timer,
            spawn_coin_over_time,
            display_score,
            fireball_movement,
            rotate_fireball,
            despawn_fireball,
            check_collision
            ).run_if(in_state(GameState::Playing)))
        .run();
}

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera).insert(PlayingComponents());

    let texture = asset_server.load("spaceship.png");

    commands.spawn((
        SpriteBundle {
            texture,
            sprite: Sprite {
                    custom_size: Some(Vec2::new(20.0, 18.0)),
                    ..default()
            },
            ..default()
        },
        Player { 
            speed: 80.0,
            score: 0,
        },
    ))
    .insert(PlayingComponents());
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
        if transform.translation.y < -120.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn check_collision(
    query_player: Query<&Transform, With<Player>>,
    mut query_player_score: Query<&mut Player>,
    query_fireball: Query<(&Transform, &Fireball), With<Fireball>>,
    query_coin: Query<(Entity, &Transform), With<Coin>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<GameState>>,
    playing_items_query: Query<Entity, With<PlayingComponents>>,
    music_query: Query<Entity, With<Music>>,
) {

    for player_transform in query_player.iter() {
        for (fireball_transform, fireball) in query_fireball.iter() {
            let player_position = player_transform;
            let fireball_position = fireball_transform;
            if meteor_is_colliding(player_position, fireball_position, (fireball.fireball_size) as f64) {
                commands.spawn(AudioBundle {
                    source: asset_server.load("crash.ogg"),
                    ..default()
                });
                for playing_items in playing_items_query.iter() {
                    commands.entity(playing_items).despawn();
                }
                for music in music_query.iter() {
                    commands.entity(music).despawn();
                }
                game_state.set(GameState::StartMenu);
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

                for mut player_score in &mut query_player_score {
                    player_score.score+=1;
                }
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

fn display_score(
    mut commands: Commands,
    query_player: Query<&mut Player>,
    query_text: Query<Entity, With<Text>>,
) {
    let mut score: i8 = 0;

    for player in &query_player {
        score = player.score;
    }

    for text_item in query_text.iter() {
        commands.entity(text_item).despawn();
    }

    let string_score = score.to_string();


    commands.spawn(TextBundle::from_section(
        "Score: ",
        TextStyle {
            font_size: 20.0,

            ..default()
        })
        .with_style( Style {
            position_type: PositionType::Absolute,
            margin: UiRect::new(
                Val::Px(5.0),
                Val::Px(5.0),
                Val::Px(5.0),
                Val::Px(5.0)
            ),
            ..default()
        })
    )
    .insert(PlayingComponents());

    commands.spawn(TextBundle::from_section(
        string_score,
        TextStyle {
            font_size: 20.0,

            ..default()
        })
        .with_style( Style {
            position_type: PositionType::Absolute,
            margin: UiRect::new(
                Val::Px(80.0),
                Val::Px(5.0),
                Val::Px(5.0),
                Val::Px(5.0)
            ),
            ..default()
        })
    )
    .insert(Text());
}

pub fn tick_fireball_timer(
    mut fireball_spaw_timer: ResMut<FireballSpawnTimer>, 
    time: Res<Time>
) {
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
                transform: Transform::from_xyz(fireball_pos1, 140.0, 1.0),
                ..default()
            },
        ))
        .insert(Fireball { 
            fireball_speed: 55.0, 
            rotate_direction: 2.0, 
            fireball_size 
        })
        .insert(PlayingComponents());

        fireball_size = rand::thread_rng().gen_range(25..=60) as f32;

        commands.spawn((
            SpriteBundle {
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(fireball_size, fireball_size)),
                    ..default()
                },
                transform: Transform::from_xyz(fireball_pos2, 140.0, 1.0),
                ..default()
            },
        ))
        .insert(Fireball { 
            fireball_speed: 70.0, 
            rotate_direction: -2.0,
            fireball_size
        })
        .insert(PlayingComponents());
    }
}

fn rotate_fireball(
    time: Res<Time>, 
    mut query: Query<(&Fireball, &mut Transform)>
) {

    for (fireball, mut transform) in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_z(fireball.rotate_direction * time.delta_seconds());
    }
}

fn tick_coin_timer(
    mut coin_spaw_timer: ResMut<CoinSpawnTimer>, 
    time: Res<Time>
) {
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
        .insert( Coin {} )
        .insert(PlayingComponents());
    }
}

#[derive(Component)]
struct Player {
    pub speed: f32,
    score: i8,
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

#[derive(Component)]
pub struct Text();

impl Default for CoinSpawnTimer {
    fn default() -> CoinSpawnTimer {
        CoinSpawnTimer { 
            timer: Timer::from_seconds(COIN_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
