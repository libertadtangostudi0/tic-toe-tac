// src/board/traits.rs

use crate::board::resources::{BoardState, CurrentPlayer, Player};
use crate::core::player::CorePlayer;

/// Anything that can apply a move to the underlying core board.
pub trait ApplyCellMove {
    /// Returns true if the move was successfully applied.
    fn apply_move(&mut self, row: u8, col: u8, player: CorePlayer) -> bool;
}

/// Anything that can switch the current player turn.
pub trait SwitchPlayer {
    fn switch_player(&mut self);
}

// Implementations for current resources
impl ApplyCellMove for BoardState {
    fn apply_move(&mut self, row: u8, col: u8, player: CorePlayer) -> bool {
        self.core_board.place_mark(row, col, player)
    }
}

impl SwitchPlayer for CurrentPlayer {
    fn switch_player(&mut self) {
        self.player = match self.player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }
}
