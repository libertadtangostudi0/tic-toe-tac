// src/debug_tools/mod.rs
use bevy::prelude::*;

pub mod logging;
pub mod systems_overlay;
pub mod systems_hotkeys;


use logging::resources::{LogConfig, LogRuntimeState, LogEvent};
use logging::systems::{init_log_directory, flush_log_buffer};

use systems_overlay::draw_overlay;
use systems_hotkeys::handle_debug_hotkeys;


pub struct DebugToolsPlugin;

impl Plugin for DebugToolsPlugin {
    fn build(&self, app: &mut App) {
        app

            // Logging resources
            .init_resource::<logging::resources::LogConfig>()
            .init_resource::<logging::resources::LogRuntimeState>()
            
            // Logging events
            .add_message::<logging::resources::LogEvent>()

            // Logging systems
            .add_systems(PreStartup, init_log_directory)
            .add_systems(Update, flush_log_buffer)

            // Debug overlay and hotkeys
            .add_systems(Update, systems_overlay::draw_overlay)
            .add_systems(Update, systems_hotkeys::handle_debug_hotkeys);
    }
}
