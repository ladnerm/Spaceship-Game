use bevy::prelude::*;

pub mod components;
mod resources;
pub mod systems;

use systems::*;
use resources::*;
use crate::events::GameState;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<CoinSpawnTimer>()
		.add_systems(Update, (
			tick_coin_timer,
			spawn_coin_over_time
			)
		.run_if(in_state(GameState::Playing)));
	}
}
