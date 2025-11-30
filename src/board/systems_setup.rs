use bevy::prelude::*;

use crate::board::components::{BoardRoot, Cell};
use crate::board::resources::{BoardState, CurrentPlayer, GameConfig, GameResult};
use crate::core::board::CoreBoard;

const CELL_SIZE: f32 = 120.0;
const CELL_PADDING: f32 = 8.0;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn reset_game_state(
    config: Res<GameConfig>,
    mut board_state: ResMut<BoardState>,
    mut current_player: ResMut<CurrentPlayer>,
    mut game_result: ResMut<GameResult>,
) {
    board_state.core_board = CoreBoard::new(config.grid_size);
    current_player.player = crate::board::resources::Player::X;
    game_result.winner = None;
    game_result.is_draw = false;
}

pub fn setup_board(
    mut commands: Commands,
    config: Res<GameConfig>,
) {
    let n = config.grid_size as f32;
    let half = (n - 1.0) / 2.0;

    let board_root = commands
        .spawn((
            BoardRoot,
            Transform::default(),
            GlobalTransform::default(),
        ))
        .id();

    for row in 0..(config.grid_size as i32) {
        for col in 0..(config.grid_size as i32) {
            let row_f = row as f32;
            let col_f = col as f32;

            let x = (col_f - half) * CELL_SIZE;
            let y = (half - row_f) * CELL_SIZE;

            let color = if (row + col) % 2 == 0 {
                Color::srgb(0.15, 0.18, 0.30)
            } else {
                Color::srgb(0.10, 0.12, 0.22)
            };

            let sprite = Sprite {
                color,
                custom_size: Some(Vec2::splat(CELL_SIZE - CELL_PADDING)),
                ..Default::default()
            };

            commands.entity(board_root).with_children(|parent| {
                parent.spawn((
                    Cell {
                        row: row as u8,
                        col: col as u8,
                    },
                    sprite,
                    Transform::from_xyz(x, y, 0.0),
                ));
            });
        }
    }
}
