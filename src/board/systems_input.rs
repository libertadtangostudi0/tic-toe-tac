// src/board/systems_input.rs

use bevy::prelude::*;
use crate::board::resources::*;
use crate::core::player::CorePlayer;
use crate::input::events::CellClickedEvent;
use crate::debug_tools::logging::resources::{LogEvent, LogLevel};

pub fn apply_cell_clicks(
    mut events: EventReader<CellClickedEvent>,
    mut board_state: ResMut<BoardState>,
    mut current: ResMut<CurrentPlayer>,
    mut log_events: EventWriter<LogEvent>,
) {
    let mut move_was_made = false;
    let current_player_core: CorePlayer = current.player.into();

    for event in events.read() {
        let row = event.row;
        let col = event.col;

        let placed = board_state
            .core_board
            .place_mark(row, col, current_player_core);

        if placed {
            move_was_made = true;
            log_events.send(LogEvent {
                level: LogLevel::Info,
                message: format!(
                    "Move accepted: player {:?} -> ({}, {})",
                    current.player, row, col
                ),
            });
        } else {
            log_events.send(LogEvent {
                level: LogLevel::Debug,
                message: format!(
                    "Move rejected: player {:?} -> ({}, {})",
                    current.player, row, col
                ),
            });
        }
    }

    if move_was_made {
        // Switch player only once per frame if at least one move was valid.
        current.player = match current.player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }
}
