use bevy::prelude::*;

pub mod components;
pub mod systems;
mod resources;

use crate::astroid::resources::AstroidSpawnTimer;
use crate::events::GameState;
use crate::player::resources::HighScore;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AstroidSpawnTimer>()
            .init_resource::<HighScore>()
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(OnEnter(GameState::Menu), display_player_high_score)
            .add_systems(
                Update,
                (character_movement, check_collision, update_high_score).run_if(in_state(GameState::Playing)),
            );
    }
}
