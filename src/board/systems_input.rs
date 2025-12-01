use bevy::prelude::*;

use crate::board::components::{GameRoot, CellClickIntent};
use crate::board::resources::{BoardState, CurrentPlayer};
use crate::board::traits::{ApplyCellMove, SwitchPlayer};
use crate::core::player::CorePlayer;
use crate::input::events::CellClickedEvent;
use crate::debug_tools::logging::resources::{LogEvent, LogLevel};

/// Convert low-level CellClickedEvent into a CellClickIntent
/// attached to the GameRoot entity.
pub fn spawn_cell_click_intents(
    mut commands: Commands,
    mut events: MessageReader<CellClickedEvent>,
    game_root_query: Query<Entity, With<GameRoot>>,
) {
    // We expect exactly one GameRoot entity; if not found, just skip.
    let Ok(game_entity) = game_root_query.single() else {
        return;
    };

    for ev in events.read() {
        commands.entity(game_entity).insert(CellClickIntent {
            row: ev.row,
            col: ev.col,
        });
    }
}

/// Apply CellClickIntent to the core board state and switch player.
/// This is the core gameplay logic per click.
pub fn apply_cell_intents(
    mut commands: Commands,
    mut board_state: ResMut<BoardState>,
    mut current: ResMut<CurrentPlayer>,
    mut log_events: MessageWriter<LogEvent>,
    query: Query<(Entity, &CellClickIntent), With<GameRoot>>,
) {
    let mut move_was_made = false;
    let current_player_core: CorePlayer = current.player.into();

    for (entity, intent) in query.iter() {
        let row = intent.row;
        let col = intent.col;

        // Use trait instead of directly touching core_board.
        let placed = board_state.apply_move(row, col, current_player_core);

        if placed {
            move_was_made = true;
            log_events.write(LogEvent {
                level: LogLevel::Info,
                message: format!(
                    "Move accepted (intent): player {:?} -> ({}, {})",
                    current.player, row, col
                ),
            });
        } else {
            log_events.write(LogEvent {
                level: LogLevel::Debug,
                message: format!(
                    "Move rejected (intent): player {:?} -> ({}, {})",
                    current.player, row, col
                ),
            });
        }

        // Intent processed -> remove it
        commands.entity(entity).remove::<CellClickIntent>();
    }

    if move_was_made {
        // Use trait to switch player
        current.switch_player();
    }
}
