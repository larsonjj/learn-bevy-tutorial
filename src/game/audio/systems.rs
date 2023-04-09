use bevy::prelude::*;

use crate::loading::resources::AudioAssets;

use super::resources::*;

pub fn start_background_audio(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
    mut bg_music: ResMut<BackgroundMusicAudio>,
) {
    let weak_handle = audio.play_with_settings(
        audio_assets.background_music.clone(),
        PlaybackSettings {
            volume: 0.3,
            ..Default::default()
        },
    );

    // MUST make the handle strong so it can be used for playback functions (i.e. pause)
    // Docs: https://docs.rs/bevy/0.9.1/bevy/audio/struct.Audio.html#method.play
    bg_music.handle = audio_sinks.get_handle(weak_handle);
}

pub fn stop_background_audio(
    audio_sinks: Res<Assets<AudioSink>>,
    bg_music: Res<BackgroundMusicAudio>,
) {
    if let Some(sink) = audio_sinks.get(&bg_music.handle) {
        sink.pause()
    }
}
