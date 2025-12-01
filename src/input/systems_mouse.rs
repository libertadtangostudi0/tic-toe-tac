use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::input::events::CellClickedEvent;
use crate::board::components::Cell;
use crate::debug_tools::logging::resources::{LogEvent, LogLevel};

pub fn mouse_to_cell_clicks(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    cells: Query<(&Cell, &GlobalTransform, &Sprite)>,
    mut click_events: MessageWriter<CellClickedEvent>,
    mut log_events: MessageWriter<LogEvent>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else { return; };
    let Ok((camera, cam_tr)) = cameras.single() else { return; };
    let Some(cursor) = window.cursor_position() else { return; };
    let Ok(world_pos) = camera.viewport_to_world_2d(cam_tr, cursor) else { return; };

    let mut clicked: Option<(u8, u8)> = None;

    for (cell, tf, sprite) in cells.iter() {
        let center = tf.translation().truncate();
        let size = sprite.custom_size.unwrap_or(Vec2::splat(64.0));
        let half = size * 0.5;
        let min = center - half;
        let max = center + half;

        if world_pos.x >= min.x
            && world_pos.x <= max.x
            && world_pos.y >= min.y
            && world_pos.y <= max.y
        {
            clicked = Some((cell.row, cell.col));
            break;
        }
    }

    let Some((row, col)) = clicked else { return; };

    log_events.write(LogEvent {
        level: LogLevel::Debug,
        message: format!("Input: clicked cell ({}, {})", row, col),
    });

    click_events.write(CellClickedEvent { row, col });
}
