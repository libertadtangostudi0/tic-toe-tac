// src/input/events.rs
use bevy::prelude::*;

#[derive(Event, Message, Debug, Clone, Copy)]
pub struct CellClickedEvent {
    pub row: u8,
    pub col: u8,
}
