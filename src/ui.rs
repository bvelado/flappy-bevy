use bevy::{
    prelude::{
        info, Color, Commands, Component, DespawnRecursiveExt, Entity, Query, Res, Transform, Vec3,
    },
    text::{Text, Text2dBundle, TextAlignment, TextStyle},
};

use crate::assets::GameAssets;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum UiElement {
    MainMenuTitle,
    GameReadyLabel,
}

pub fn spawn_game_ready_label(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Ready to play",
                TextStyle {
                    font: game_assets.font.clone(),
                    font_size: 22.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            )
            .with_alignment(TextAlignment::CENTER),
            transform: Transform::from_translation(Vec3::new(0.0, 40.0, 0.0)),
            ..Default::default()
        },
        UiElement::GameReadyLabel,
    ));
}

pub fn despawn_game_ready_label(mut commands: Commands, q_element: Query<(Entity, &UiElement)>) {
    q_element
        .iter()
        .filter(|(_, &ui)| ui == UiElement::GameReadyLabel)
        .for_each(|(e, _)| {
            info!("Destroying GameReadyLabel UiElement");
            commands.entity(e).despawn_recursive();
        });
}
