use bevy::prelude::*;

/// Top-level application configuration shared across subsystems.
/// This resource contains metadata and app-wide parameters unrelated to gameplay/UI/themes/etc.
#[derive(Resource)]
pub struct AppConfig {
    /// Title of the application window.
    pub window_title: String,

    /// Application version (comes from Cargo package metadata).
    pub version: String,

    /// Debug mode flag. True in debug builds by default.
    pub dev_mode: bool,

    /// Whether debug overlay should be enabled.
    pub debug_overlay_enabled: bool,

    /// Target FPS for the application. None = unlimited.
    pub target_fps: Option<u32>,
}

impl Default for AppConfig {
    fn default() -> Self {
        let dev = cfg!(debug_assertions);

        Self {
            window_title: "Tic Tac Toe".to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            dev_mode: dev,
            debug_overlay_enabled: dev,
            target_fps: Some(144),
        }
    }
}
