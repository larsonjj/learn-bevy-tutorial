use crate::enemy::EnemyHitWallEvent;
use crate::loading::AudioAssets;
use crate::player::PlayerDiedEvent;
use crate::GameState;
use bevy::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audiocargo
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_audio.in_schedule(OnEnter(GameState::Playing)))
            .add_system(control_player_died_sound.in_set(OnUpdate(GameState::Playing)))
            .add_system(control_enemy_wall_hit_sound.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Resource)]
struct BackgroundMusicAudio(Handle<AudioSource>);

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play_with_settings(
        audio_assets.background_music.clone(),
        PlaybackSettings {
            volume: 0.3,
            ..Default::default()
        },
    );
}

fn control_enemy_wall_hit_sound(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut enemy_hit_wall_events: EventReader<EnemyHitWallEvent>,
) {
    if !enemy_hit_wall_events.is_empty() {
        // This prevents events staying active on the next frame.
        enemy_hit_wall_events.clear();

        // Randomely play one of the two sounds
        let sound_effect = if rand::random() {
            audio_assets.enemy_hit_wall_1.clone()
        } else {
            audio_assets.enemy_hit_wall_2.clone()
        };
        audio.play_with_settings(
            sound_effect,
            PlaybackSettings {
                volume: 0.5,
                ..Default::default()
            },
        );
    }
}

fn control_player_died_sound(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut player_died_events: EventReader<PlayerDiedEvent>,
) {
    if !player_died_events.is_empty() {
        // This prevents events staying active on the next frame.
        player_died_events.clear();

        // Randomely play one of the two sounds
        audio.play_with_settings(
            audio_assets.player_died.clone(),
            PlaybackSettings {
                volume: 0.5,
                ..Default::default()
            },
        );
    }
}
