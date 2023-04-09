use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct BackgroundMusicAudio {
    pub handle: Handle<AudioSink>,
}
