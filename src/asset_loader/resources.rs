use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/font_space/pixeloid_mono.ttf")]
    pub pixeliod_mono: Handle<Font>,
    #[asset(path = "fonts/font_space/pixeloid_sans_bold.ttf")]
    pub pixeliod_bold: Handle<Font>,
    #[asset(path = "fonts/font_space/pixeloid_sans.ttf")]
    pub pixeliod_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/chosic/fluffing_a_duck.ogg")]
    pub background_music: Handle<AudioSource>,
    #[asset(path = "audio/kenney/interface/pluck_001.ogg")]
    pub enemy_hit_wall_1: Handle<AudioSource>,
    #[asset(path = "audio/kenney/interface/pluck_002.ogg")]
    pub enemy_hit_wall_2: Handle<AudioSource>,
    #[asset(path = "audio/kenney/sci_fi/explosionCrunch_000.ogg")]
    pub player_died: Handle<AudioSource>,
    #[asset(path = "audio/kenney/impact/impactMetal_heavy_001.ogg")]
    pub star_pickup: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/kenney/rolling_ball/ball_blue_large.png")]
    pub texture_ball: Handle<Image>,
    #[asset(path = "textures/kenney/rolling_ball/ball_red_large.png")]
    pub enemy_ball: Handle<Image>,
    #[asset(path = "textures/kenney/rolling_ball/star.png")]
    pub star: Handle<Image>,
}
