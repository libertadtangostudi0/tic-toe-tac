use bevy::prelude::*;
use crate::app_state::AppState;

#[derive(Component)]
pub struct GameOverUI;

pub fn setup_ui(mut commands: Commands) {
    // Winner / draw screen
}

pub fn update(
    mut next: ResMut<NextState<AppState>>,
) {
    // Restart / back to menu
}

pub fn cleanup(mut commands: Commands, q: Query<Entity, With<GameOverUI>>) {
    // Remove game over UI
}
