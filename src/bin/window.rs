// src/bin/window.rs

use bevy::prelude::*;
use tictactoe::app_state::AppState;
use tictactoe::board::CoreGamePlugin;
use tictactoe::input::InputPlugin;
use tictactoe::debug_tools::DebugToolsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(InputPlugin)
        .add_plugins(CoreGamePlugin)
        .add_plugins(DebugToolsPlugin)
        .run();
}
