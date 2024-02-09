use bevy::prelude::*;

pub mod components;
pub mod systems;

use crate::astroid::resources::AstroidSpawnTimer;
use crate::events::GameState;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app:&mut App) {
		app.init_resource::<AstroidSpawnTimer>()
		.add_systems(OnEnter(GameState::Playing), spawn_player)
		.add_systems(Update, (
			character_movement,
			check_collision
			).run_if(in_state(GameState::Playing))
		);
	}
}