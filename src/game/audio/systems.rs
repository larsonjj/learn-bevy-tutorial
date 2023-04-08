use bevy::prelude::*;

use crate::loading::resources::AudioAssets;

pub fn start_background_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play_with_settings(
        audio_assets.background_music.clone(),
        PlaybackSettings {
            volume: 0.3,
            ..Default::default()
        },
    );
}
