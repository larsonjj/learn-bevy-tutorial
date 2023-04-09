// References
// 1. UI Z-Index
// https://github.com/bevyengine/bevy/blob/latest/examples/ui/z_index.rs

use bevy::prelude::*;

use crate::game::ui::pause_menu::components::{
    MainMenuButton, PauseMenu, QuitButton, ResumeButton,
};
use crate::game::ui::pause_menu::styles::*;
use crate::loading::resources::FontAssets;

pub fn spawn_pause_menu(mut commands: Commands, font_assets: Res<FontAssets>) {
    println!("Spawning Pause Menu");
    build_pause_menu(&mut commands, &font_assets);
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    println!("Despawning Pause Menu");
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

// System Piping Example
pub fn build_pause_menu(commands: &mut Commands, font_assets: &Res<FontAssets>) -> Entity {
    let pause_menu_entity = commands
        .spawn((
            NodeBundle {
                style: PAUSE_MENU_STYLE,
                z_index: ZIndex::Local(1), // See Ref. 1
                ..default()
            },
            PauseMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: PAUSE_MENU_CONTAINER_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Pause Menu",
                                get_title_text_style(&font_assets),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // Resume Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            ResumeButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Resume",
                                        get_button_text_style(&font_assets),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Main Menu Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MainMenuButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Main Menu",
                                        get_button_text_style(&font_assets),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    #[cfg(not(target_family = "wasm"))]
                    {
                        // Quit Button
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: BUTTON_STYLE,
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                QuitButton {},
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    style: Style { ..default() },
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            "Quit",
                                            get_button_text_style(&font_assets),
                                        )],
                                        alignment: TextAlignment::Center,
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                    }
                });
        })
        .id();

    pause_menu_entity
}
