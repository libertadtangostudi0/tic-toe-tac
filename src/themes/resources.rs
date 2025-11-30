use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ThemeId {
    Classic,
    Dark,
    Neon,
}

#[derive(Clone)]
pub struct ThemeConfig {
    pub id: ThemeId,
    // Board visuals
    pub background_color: Color,
    pub grid_color: Color,
    // X/O visuals
    pub x_color: Color,
    pub o_color: Color,
    // UI colors
    pub ui_background_color: Color,
    pub ui_text_color: Color,
    // Asset handles can be added later (sprites, fonts, etc.)
    // pub x_texture: Handle<Image>,
    // pub o_texture: Handle<Image>,
}

#[derive(Resource)]
pub struct ThemeLibrary {
    pub themes: HashMap<ThemeId, ThemeConfig>,
}

impl Default for ThemeLibrary {
    fn default() -> Self {
        // Implementation will be provided later
        Self {
            themes: HashMap::new(),
        }
    }
}

#[derive(Resource)]
pub struct ActiveTheme {
    pub id: ThemeId,
}

impl Default for ActiveTheme {
    fn default() -> Self {
        // Implementation will be provided later (e.g. ThemeId::Classic)
        Self {
            id: ThemeId::Classic,
        }
    }
}
