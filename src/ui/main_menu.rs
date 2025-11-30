use bevy::prelude::*;
use crate::app_state::AppState;
use crate::themes::resources::{ActiveTheme, ThemeId};

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct ThemeButton {
    pub theme_id: ThemeId,
}

pub fn setup_ui(mut commands: Commands) {
    // Spawn main menu UI:
    // - Title
    // - StartGameButton
    // - Several ThemeButton entities bound to ThemeId variants
}

pub fn update(
    // Button interactions
    mut next_state: ResMut<NextState<AppState>>,
    mut active_theme: ResMut<ActiveTheme>,
    // queries with Interaction, ThemeButton, StartGameButton, etc.
) {
    // 1) Start game: set AppState::InGame
    // 2) Theme selection: set active_theme.id = clicked ThemeButton.theme_id
}

pub fn cleanup(
    mut commands: Commands,
    q_menu: Query<Entity, With<MainMenuUI>>,
) {
    // Despawn full main menu UI hierarchy
}
