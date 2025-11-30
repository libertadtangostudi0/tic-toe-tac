use bevy::prelude::*;
use crate::input::events::CellClickedEvent;

pub fn mouse_to_cell_clicks(
    // windows, camera, cursor position, etc.
    mut events: EventWriter<CellClickedEvent>,
) {
    // Convert mouse position to CellClickedEvent
}
