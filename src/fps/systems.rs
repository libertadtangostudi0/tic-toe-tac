use bevy::prelude::*;
use crate::config::AppConfig;

pub fn fps_limiter(
    config: Res<AppConfig>,
    mut time: ResMut<Time<Virtual>>,
) {
    if let Some(fps) = config.target_fps {
        let min_dt = 1.0 / fps as f32;

        // Busy-wait pacing. Later можно заменить на sleep.
        let now = std::time::Instant::now();
        while now.elapsed().as_secs_f32() < min_dt {
            // spin
        }

        // Optionally clamp the delta so systems using delta don't see tiny values
        time.advance_by(std::time::Duration::from_secs_f32(min_dt));
    }
}
