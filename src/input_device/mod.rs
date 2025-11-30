use bevy::prelude::*;

pub mod events;
pub mod systems_mouse;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<events::CellClickedEvent>()
            .add_systems(
                Update,
                systems_mouse::mouse_to_cell_clicks,
            );
    }
}
