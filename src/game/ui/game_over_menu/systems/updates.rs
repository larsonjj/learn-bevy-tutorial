use bevy::prelude::*;

use crate::game::{resources::Score, ui::game_over_menu::components::FinalScoreText};

pub fn update_final_score_text(
    score: Res<Score>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("Final Score: {}", score.value.to_string());
    }
}
