// src/board/events.rs
use bevy::prelude::*;

#[derive(Message, Debug, Clone, Copy)]
pub struct CellClickedEvent {
    pub row: u8,
    pub col: u8,
}
