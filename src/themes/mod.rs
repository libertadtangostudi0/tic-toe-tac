use bevy::prelude::*;

pub mod resources;
pub mod systems_load;
pub mod systems_apply;

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<resources::ThemeLibrary>()
            .init_resource::<resources::ActiveTheme>()
            .add_systems(Startup, systems_load::init_themes)
            .add_systems(Update, systems_apply::detect_theme_change);
    }
}
