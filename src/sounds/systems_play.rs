use bevy::prelude::*;
use super::resources::{SoundLibrary, SoundSettings, SoundId};

pub struct PlaySoundEvent {
    pub id: SoundId,
}

pub fn play_sound_system(
    mut events: EventReader<PlaySoundEvent>,
    sound_library: Res<SoundLibrary>,
    settings: Res<SoundSettings>,
    audio: Res<Audio>,
) {
    // For each event:
    // - check settings.muted
    // - get handle from sound_library.sounds
    // - play via audio
}
