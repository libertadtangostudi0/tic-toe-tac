use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems_setup;
// pub mod systems_input;
// pub mod systems_logic;
pub mod systems_cleanup;

use crate::app_state::AppState;
use resources::{BoardState, CurrentPlayer, GameConfig, GameResult};

pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Core game resources
            .init_resource::<GameConfig>()
            .init_resource::<BoardState>()
            .init_resource::<CurrentPlayer>()
            .init_resource::<GameResult>()

            // Camera
            .add_systems(Startup, systems_setup::setup_camera)

            // Board spawn / reset on entering InGame
            .add_systems(
                OnEnter(AppState::InGame),
                (
                    systems_setup::reset_game_state,
                    systems_setup::setup_board,
                )
                    .chain(),
            )

            // TODO: enable gameplay systems later
            /*
            .add_systems(
                Update,
                (
                    systems_input::apply_cell_clicks,
                    systems_logic::sync_visuals_with_board,
                    systems_logic::evaluate_game_state,
                    systems_logic::update_current_player,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            */

            // Cleanup board on exit
            .add_systems(
                OnExit(AppState::InGame),
                systems_cleanup::cleanup_board,
            );
    }
}
