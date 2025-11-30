use bevy::prelude::*;
use super::resources::{SoundLibrary, SoundId};

pub fn load_sounds(
    asset_server: Res<AssetServer>,
    mut sound_library: ResMut<SoundLibrary>,
) {
    // Example:
    // let click = asset_server.load("audio/button_click.ogg");
    // sound_library.sounds.insert(SoundId::ButtonClick, click);
}
