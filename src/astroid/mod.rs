use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use systems::*;
use crate::events::GameState;
use crate::astroid::resources::AstroidSpawnTimer;

pub struct AstroidPlugin;

impl Plugin for AstroidPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<AstroidSpawnTimer>()
		.add_systems(Update, (
			spawn_astroid_over_time,
			tick_astroid_timer,
			astroid_movement,
			rotate_astroid,
			despawn_astroid
			).run_if(in_state(GameState::Playing)));
	}
}