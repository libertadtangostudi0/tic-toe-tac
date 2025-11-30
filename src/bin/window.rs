// src/bin/window.rs

use bevy::prelude::*;
use tictactoe::app_state::AppState;
use tictactoe::board::CoreGamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(CoreGamePlugin)
        .run();
}
