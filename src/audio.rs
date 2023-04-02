use crate::enemy::EnemyDirectionChangedEvent;
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audiocargo
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_audio.in_schedule(OnEnter(GameState::Playing)))
            .add_system(control_enemy_direction_changed_sound.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Resource)]
struct BackgroundMusicAudio(Handle<AudioSource>);

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play(audio_assets.background_music.clone());
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
        audio.play(audio_assets.enemy_direction_changed_1.clone());
    }
}
