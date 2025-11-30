// src/board/systems_logic.rs

use bevy::prelude::*;
use crate::board::components::*;
use crate::board::resources::*;
use crate::core::rules::check_winner;
use crate::app_state::AppState;
use crate::debug_tools::logging::resources::{LogEvent, LogLevel};

pub fn sync_visuals_with_board(
    _board_state: Res<BoardState>,
    _q_cells: Query<(&Cell, Option<&mut CellOwner>)>,
    _commands: Commands,
) {
    // TODO: update visuals later.
}

pub fn evaluate_game_state(
    board_state: Res<BoardState>,
    mut game_result: ResMut<GameResult>,
    mut next_state: ResMut<NextState<AppState>>,
    mut log_events: EventWriter<LogEvent>,
) {
    let core_result = check_winner(&board_state.core_board);

    if core_result.winner.is_none() && !core_result.is_draw {
        return;
    }

    game_result.winner = core_result.winner.map(Into::into);
    game_result.is_draw = core_result.is_draw;

    if let Some(winner) = game_result.winner {
        log_events.send(LogEvent {
            level: LogLevel::Info,
            message: format!("Game over: winner = {:?}", winner),
        });
    } else if game_result.is_draw {
        log_events.send(LogEvent {
            level: LogLevel::Info,
            message: "Game over: draw".to_string(),
        });
    }

    next_state.set(AppState::GameOver);
}
