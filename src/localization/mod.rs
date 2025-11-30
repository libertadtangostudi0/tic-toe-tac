use bevy::prelude::*;

pub mod resources;
pub mod systems_load;
pub mod systems_apply;

pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<resources::Localization>()
            .init_resource::<resources::CurrentLanguage>()
            .add_systems(Startup, systems_load::load_localization)
            .add_systems(Update, systems_apply::apply_localization_to_ui);
    }
}
