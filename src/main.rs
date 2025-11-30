// mod config;
// mod fps;
// mod app_state;
// mod core;
// mod board;

// UI / input / themes / sounds / settings / localization — turn off
// mod input;
// mod ui;
// mod themes;
// mod sounds;
// mod settings;
// mod localization;
// mod debug_tools;
// mod utils;

use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};

// Import modules from the library crate
use tictactoe::config::AppConfig;
use tictactoe::app_state::AppState;
use tictactoe::board::CoreGamePlugin;
use tictactoe::fps::FPSLimiterPlugin;

fn main() {
    let app_config = AppConfig::default();
    let window_title = app_config.window_title.clone();

    App::new()
        .insert_resource(app_config)

        // Window setup
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: window_title,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )

        // App state
        .init_state::<AppState>()

        // === Enabled only minimal core ===
        .add_plugins(board::CoreGamePlugin)
        .add_plugins(FPSLimiterPlugin)

        // === Leave the rest here for future ===
        /*
        .add_plugins(settings::SettingsPlugin)
        .add_plugins(localization::LocalizationPlugin)
        .add_plugins(themes::ThemePlugin)
        .add_plugins(sounds::SoundPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(debug_tools::DebugToolsPlugin)
        */

        .run();
}
