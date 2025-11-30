use bevy::prelude::*;

#[derive(Component)]
pub struct InGameUI;

pub fn setup_ui(mut commands: Commands) {
    // HUD (player turn etc.)
}

pub fn cleanup(mut commands: Commands, q: Query<Entity, With<InGameUI>>) {
    // Remove HUD
}
