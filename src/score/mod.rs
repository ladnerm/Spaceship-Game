use bevy::prelude::*;

mod systems;
pub mod resources;
pub mod components;

use crate::events::GameState;
use crate::score::resources::HighScore;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScore>()
            .add_systems(Update, (display_score, update_high_score).run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Menu), display_high_score);

    }
}
