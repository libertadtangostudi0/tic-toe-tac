// src/board/resources.rs

use bevy::prelude::*;
use crate::core::board::CoreBoard;
use crate::core::player::CorePlayer;

#[derive(Resource)]
pub struct GameConfig {
    pub grid_size: u8,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self { grid_size: 3 }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Player {
    X,
    O,
}

impl From<Player> for CorePlayer {
    fn from(p: Player) -> Self {
        match p {
            Player::X => CorePlayer::X,
            Player::O => CorePlayer::O,
        }
    }
}

#[derive(Resource)]
pub struct BoardState {
    pub core_board: CoreBoard,
}

impl Default for BoardState {
    fn default() -> Self {
        Self {
            core_board: CoreBoard::new(3),
        }
    }
}

#[derive(Resource)]
pub struct CurrentPlayer {
    pub player: Player,
}

impl Default for CurrentPlayer {
    fn default() -> Self {
        Self { player: Player::X }
    }
}

#[derive(Resource)]
pub struct GameResult {
    pub winner: Option<Player>,
    pub is_draw: bool,
}

impl Default for GameResult {
    fn default() -> Self {
        Self {
            winner: None,
            is_draw: false,
        }
    }
}
