use crate::enemy::EnemyDirectionChangedEvent;
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audiocargo
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system(start_audio.in_schedule(OnEnter(GameState::Playing)))
            .add_system(play_background_music.in_set(OnUpdate(GameState::Playing)))
            .add_system(control_enemy_direction_changed_sound.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Resource)]
struct BackgroundMusicAudio(Handle<AudioInstance>);

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    let background_music_handle = audio
        .play(audio_assets.background_music.clone())
        .looped()
        .with_volume(0.3)
        .handle();

    // Insert handles into the world so that we can control the audio later
    commands.insert_resource(BackgroundMusicAudio(background_music_handle));
}

fn control_enemy_direction_changed_sound(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut enemy_direction_changed_events: EventReader<EnemyDirectionChangedEvent>,
) {
    if !enemy_direction_changed_events.is_empty() {
        println!("Enemy direction changed event received");
        // This prevents events staying active on the next frame.
        enemy_direction_changed_events.clear();
        audio
            .play(audio_assets.enemy_direction_changed_1.clone())
            .with_volume(0.3);
    }
}

fn play_background_music(
    audio: Res<BackgroundMusicAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        match instance.state() {
            PlaybackState::Paused { .. } => {
                instance.resume(AudioTween::default());
            }
            PlaybackState::Playing { .. } => {
                instance.pause(AudioTween::default());
            }
            _ => {}
        }
    }
}
