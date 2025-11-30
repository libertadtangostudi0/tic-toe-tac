use bevy::prelude::*;
use crate::app_state::AppState;
use crate::themes::resources::{ActiveTheme, ThemeId};
use crate::sounds::systems_play::PlaySoundEvent;
use crate::sounds::resources::SoundId;

#[derive(Component)]
pub struct ButtonAction {
    pub kind: ButtonActionKind,
}

pub enum ButtonActionKind {
    StartGame,
    BackToMainMenu,
    RestartGame,
    ChangeTheme(ThemeId),
    ToggleMute,
    QuitGame,
}

pub fn handle_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<Button>)
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut active_theme: ResMut<ActiveTheme>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
) {
    // React to button clicks: change state, theme, send sounds
}
