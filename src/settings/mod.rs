use bevy::prelude::*;

pub mod resources;
pub mod systems_load;
pub mod systems_save;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<resources::Settings>()
            .add_systems(Startup, systems_load::load_settings)
            .add_systems(Update, systems_save::save_settings_on_change);
    }
}
