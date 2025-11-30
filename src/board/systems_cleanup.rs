use bevy::prelude::*;
use crate::board::components::*;

pub fn cleanup_board(
    mut commands: Commands,
    q_root: Query<Entity, With<BoardRoot>>,
    q_cells: Query<Entity, With<Cell>>,
) {
    // Despawn board and cells
}
