use bevy::prelude::*;
pub mod systems;

pub struct FPSLimiterPlugin;

impl Plugin for FPSLimiterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::fps_limiter);
    }
}
