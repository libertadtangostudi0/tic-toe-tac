// src/board/systems_state.rs
use bevy::prelude::*;

use crate::board::resources::{BoardState, CurrentPlayer, GameConfig, GameResult, Player};
use crate::core::board::CoreBoard;

/// Reset core game state when a new game starts.
pub fn reset_game_state(
    config: Res<GameConfig>,
    mut board_state: ResMut<BoardState>,
    mut current_player: ResMut<CurrentPlayer>,
    mut game_result: ResMut<GameResult>,
) {
    board_state.core_board = CoreBoard::new(config.grid_size);
    current_player.player = Player::X;
    game_result.winner = None;
    game_result.is_draw = false;
}
