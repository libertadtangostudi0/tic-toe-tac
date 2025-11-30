use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::update_animations);
    }
}
