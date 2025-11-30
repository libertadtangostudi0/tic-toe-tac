use bevy::prelude::*;

use crate::app_state::AppState;
use crate::board;
use crate::ui;
use crate::themes;
use crate::sounds;

pub struct TicTacToePlugin;

impl Plugin for TicTacToePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<AppState>()
            .add_plugins(themes::ThemePlugin)
            .add_plugins(sounds::SoundPlugin)
            .add_systems(Startup, setup_camera)
            // Global button handling (works in all states where buttons exist)
            .add_systems(Update, ui::buttons::handle_button_interactions)
            // Main menu
            .add_systems(OnEnter(AppState::MainMenu), ui::main_menu::setup_ui)
            .add_systems(
                Update,
                ui::main_menu::update.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), ui::main_menu::cleanup)
            // In-game
            .add_systems(
                OnEnter(AppState::InGame),
                (
                    board::systems_setup::setup_board,
                    board::systems_setup::reset_game_state,
                    ui::in_game_ui::setup_ui,
                ).chain(),
            )
            .add_systems(
                Update,
                (
                    board::systems_input::handle_clicks,
                    board::systems_logic::update_visuals,
                    board::systems_logic::check_game_over,
                    board::systems_logic::switch_turns,
                    themes::systems_apply::apply_theme_to_board,
                    themes::systems_apply::apply_theme_to_in_game_ui,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                OnExit(AppState::InGame),
                (
                    board::systems_cleanup::cleanup_board,
                    ui::in_game_ui::cleanup,
                ).chain(),
            )
            // Game over
            .add_systems(OnEnter(AppState::GameOver), ui::game_over_ui::setup_ui)
            .add_systems(
                Update,
                (
                    ui::game_over_ui::update,
                    themes::systems_apply::apply_theme_to_game_over_ui,
                )
                    .run_if(in_state(AppState::GameOver)),
            )
            .add_systems(OnExit(AppState::GameOver), ui::game_over_ui::cleanup);
    }
}

fn setup_camera(mut commands: Commands) {
    // Spawn 2D camera for board and UI
}
