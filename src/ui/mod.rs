use bevy::prelude::*;

pub mod main_menu;
pub mod in_game_ui;
pub mod game_over_ui;
pub mod buttons;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                buttons::handle_button_interactions,
            )
            .add_systems(
                OnEnter(crate::app_state::AppState::MainMenu),
                main_menu::setup_ui,
            )
            .add_systems(
                Update,
                main_menu::update.run_if(in_state(crate::app_state::AppState::MainMenu)),
            )
            .add_systems(
                OnExit(crate::app_state::AppState::MainMenu),
                main_menu::cleanup,
            )
            .add_systems(
                OnEnter(crate::app_state::AppState::InGame),
                in_game_ui::setup_ui,
            )
            .add_systems(
                OnExit(crate::app_state::AppState::InGame),
                in_game_ui::cleanup,
            )
            .add_systems(
                OnEnter(crate::app_state::AppState::GameOver),
                game_over_ui::setup_ui,
            )
            .add_systems(
                Update,
                game_over_ui::update.run_if(in_state(crate::app_state::AppState::GameOver)),
            )
            .add_systems(
                OnExit(crate::app_state::AppState::GameOver),
                game_over_ui::cleanup,
            );
    }
}
