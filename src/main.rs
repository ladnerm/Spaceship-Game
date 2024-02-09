use bevy::prelude::*;

pub mod components;
mod systems;
mod events;

use systems::*;
use crate::events::GameState;

mod player;
pub mod astroid;
pub mod score;
pub mod coin;

use astroid::AstroidPlugin;
use coin::CoinPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;

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
        .add_state::<GameState>()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(OnEnter(GameState::StartMenu), (setup_menu, play_music))
        .add_systems(Update, state_transition)
        .add_systems(OnEnter(GameState::Playing), camera_setup)
        .add_plugins(PlayerPlugin)
        .add_plugins(AstroidPlugin)
        .add_plugins(CoinPlugin)
        .add_plugins(ScorePlugin)
        .run();
}


