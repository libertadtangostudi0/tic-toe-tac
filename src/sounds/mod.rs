use bevy::prelude::*;

pub mod resources;
pub mod systems_load;
pub mod systems_play;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<resources::SoundLibrary>()
            .init_resource::<resources::SoundSettings>()
            .add_event::<systems_play::PlaySoundEvent>()
            .add_systems(Startup, systems_load::load_sounds)
            .add_systems(Update, systems_play::play_sound_system);
    }
}
