use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::components::*;
use crate::events::*;

pub fn play_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("WMusic.ogg"),
        ..default()
    })
    .insert(Music);
}

pub fn setup_menu(mut commands: Commands) {
    
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

pub fn state_transition(
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

pub fn camera_setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera).insert(PlayingComponents());
}
