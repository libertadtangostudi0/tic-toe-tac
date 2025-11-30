use bevy::prelude::*;

pub mod logging;
pub mod systems_overlay;
pub mod systems_hotkeys;

pub struct DebugToolsPlugin;

impl Plugin for DebugToolsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Logging resources and events
            .init_resource::<logging::resources::LogConfig>()
            .init_resource::<logging::resources::LogRuntimeState>()
            .add_event::<logging::resources::LogEvent>()
            .add_systems(Startup, logging::systems::init_log_directory)
            .add_systems(Update, logging::systems::flush_log_buffer)
            // Debug overlay and hotkeys
            .add_systems(Update, systems_overlay::draw_overlay)
            .add_systems(Update, systems_hotkeys::handle_debug_hotkeys);
    }
}
