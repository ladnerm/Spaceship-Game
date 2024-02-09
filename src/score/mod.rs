use bevy::prelude::*;

mod systems;

use systems::*;
use crate::events::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, display_score.run_if(in_state(GameState::Playing)));
	}
}