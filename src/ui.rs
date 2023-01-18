use bevy::{
    prelude::{
        info, Changed, Color, Commands, Component, DespawnRecursiveExt, Entity, Query, Res,
        Transform, Vec3,
    },
    text::{Text, Text2dBundle, TextAlignment, TextStyle},
};

use crate::{assets::GameAssets, consts::GAME_HEIGHT, player::PlayerScore};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiElement {
    GameReadyLabel,
    GameScore,
}
trait UniqueIdentifierValue {}
impl UniqueIdentifierValue for UiElement {}

pub fn spawn_game_ready_label(commands: Commands, game_assets: Res<GameAssets>) {
    spawn_ui_text(
        commands,
        SpawnUiTextArgs {
            ui_element: UiElement::GameReadyLabel,
            text: Text::from_section(
                "Ready to play",
                TextStyle {
                    font: game_assets.font.clone(),
                    font_size: 22.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::CENTER),
            position: Vec3::new(0.0, 40.0, 0.0),
        },
    );
}

pub fn spawn_game_score(
    commands: Commands,
    game_assets: Res<GameAssets>,
    player_score: Res<PlayerScore>,
) {
    spawn_ui_text(
        commands,
        SpawnUiTextArgs {
            ui_element: UiElement::GameScore,
            text: Text::from_section(
                player_score.value.to_string(),
                TextStyle {
                    font: game_assets.font.clone(),
                    font_size: 22.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::TOP_CENTER),
            position: Vec3::new(0.0, GAME_HEIGHT / 2.0 - 11.0, 0.0),
        },
    );
}

pub fn despawn_game_ready_label(commands: Commands, q_element: Query<(Entity, &UiElement)>) {
    despawn_ui_element_entities(commands, UiElement::GameReadyLabel, q_element);
}

pub fn despawn_game_score(commands: Commands, q_element: Query<(Entity, &UiElement)>) {
    despawn_ui_element_entities(commands, UiElement::GameScore, q_element);
}

pub fn spawn_ui_text(mut commands: Commands, args: SpawnUiTextArgs) {
    commands.spawn((
        Text2dBundle {
            text: args.text,
            transform: Transform::from_translation(args.position),
            ..Default::default()
        },
        args.ui_element,
    ));
}

pub fn despawn_ui_element_entities(
    mut commands: Commands,
    ui_element: UiElement,
    q_element: Query<(Entity, &UiElement)>,
) {
    q_element
        .iter()
        .filter(|(_, &ui)| ui == ui_element)
        .for_each(|(e, _)| {
            commands.entity(e).despawn_recursive();
        });
}

pub fn update_player_score_label(
    player_score: Res<PlayerScore>,
    mut q_elements: Query<(&UiElement, &mut Text)>,
) {
    if player_score.is_changed() {
        for (ui, mut text) in q_elements.iter_mut() {
            match ui {
                UiElement::GameScore => {
                    text.sections.first_mut().unwrap().value = player_score.value.to_string();
                }
                _ => {}
            }
        }
    }
}

pub struct SpawnUiTextArgs {
    ui_element: UiElement,
    text: Text,
    position: Vec3,
}
