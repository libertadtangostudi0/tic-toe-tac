use bevy::prelude::*;

use super::resources::{ActiveTheme, ThemeLibrary, ThemeId};
use crate::board::components::BoardRoot;
use crate::ui::in_game_ui::InGameUI;
use crate::ui::game_over_ui::GameOverUI;

/// Optional helper to detect theme change and trigger updates
pub fn detect_theme_change(
    active_theme: Res<ActiveTheme>,
) {
    // React to changes in ActiveTheme if needed (e.g. via is_changed())
}

pub fn apply_theme_to_board(
    active_theme: Res<ActiveTheme>,
    theme_library: Res<ThemeLibrary>,
    mut q_board_root: Query<&mut Sprite, With<BoardRoot>>,
) {
    // Apply board colors, grid colors based on active theme
}

pub fn apply_theme_to_in_game_ui(
    active_theme: Res<ActiveTheme>,
    theme_library: Res<ThemeLibrary>,
    mut q_ui_nodes: Query<&mut BackgroundColor, With<InGameUI>>,
    mut q_text: Query<&mut Text, With<InGameUI>>,
) {
    // Apply UI background and text colors
}

pub fn apply_theme_to_game_over_ui(
    active_theme: Res<ActiveTheme>,
    theme_library: Res<ThemeLibrary>,
    mut q_ui_nodes: Query<&mut BackgroundColor, With<GameOverUI>>,
    mut q_text: Query<&mut Text, With<GameOverUI>>,
) {
    // Apply UI background and text colors for game over screen
}
