use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SoundId {
    ButtonClick,
    CellClick,
    Win,
    Lose,
    Draw,
    BackgroundMusic,
}

#[derive(Resource)]
pub struct SoundLibrary {
    pub sounds: HashMap<SoundId, Handle<AudioSource>>,
}

impl Default for SoundLibrary {
    fn default() -> Self {
        Self {
            sounds: HashMap::new(),
        }
    }
}

#[derive(Resource)]
pub struct SoundSettings {
    pub master_volume: f32,
    pub muted: bool,
}

impl Default for SoundSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            muted: false,
        }
    }
}
