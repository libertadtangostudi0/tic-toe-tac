// src/board/mod.rs
use bevy::prelude::*;

pub mod events;
pub mod components;
pub mod resources;
pub mod systems_setup;
pub mod systems_input;
pub mod systems_cleanup;
pub mod systems_state;
pub mod traits;

use crate::app_state::AppState;
use resources::{BoardState, CurrentPlayer, GameConfig, GameResult};

// input
use crate::input::events::CellClickedEvent;
use crate::board::systems_input::{spawn_cell_click_intents, apply_cell_intents};
// debug
use crate::debug_tools::logging::resources::LogEvent;

pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Core game resources
            .init_resource::<GameConfig>()
            .init_resource::<BoardState>()
            .init_resource::<CurrentPlayer>()
            .init_resource::<GameResult>()

            // Messages used by board systems
            .add_message::<CellClickedEvent>()

            // Camera
            .add_systems(Startup, systems_setup::setup_camera)

            // Board spawn / reset on entering InGame
            .add_systems(
                OnEnter(AppState::InGame),
                (
                    systems_state::reset_game_state,
                    systems_setup::setup_board,
                )
                    .chain(),
            )

            // Intent-based gameplay pipeline in InGame
            .add_systems(
                Update,
                (
                    spawn_cell_click_intents,
                    apply_cell_intents,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            )

            // Cleanup board on exit
            .add_systems(
                OnExit(AppState::InGame),
                systems_cleanup::cleanup_board,
            );
    }
}
