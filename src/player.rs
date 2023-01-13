use bevy::{
    prelude::{
        Assets, Commands, Component, Input, KeyCode, MouseButton, Query, Res, ResMut, Vec2, With,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Timer, TimerMode},
};
use bevy_rapier2d::prelude::{Ccd, Collider, CollisionGroups, GravityScale, RigidBody, Velocity};

use crate::{
    animations::{AnimationIndices, AnimationTimer},
    consts::{COLLISION_GROUP_DEATH, COLLISION_GROUP_PLAYER},
};
use crate::{assets::GameAssets, consts::JUMP_IMPULSE_VALUE};

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_atlas = TextureAtlas::from_grid(
        game_assets.characters_image.clone(),
        Vec2::new(24.0, 24.0),
        9,
        3,
        Some(Vec2::splat(2.0)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices {
        first: 24,
        last: 26,
    };
    commands.spawn((
        Player,
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            ..Default::default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.16, TimerMode::Repeating)),
        Collider::ball(10.0),
        RigidBody::Dynamic,
        Ccd::enabled(),
        GravityScale(1.0),
        CollisionGroups::new(COLLISION_GROUP_PLAYER, COLLISION_GROUP_DEATH),
        Velocity::zero(),
    ));
}

pub fn player_jump(
    mut q_player: Query<&mut Velocity, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) || mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(mut vel) = q_player.get_single_mut() {
            vel.linvel = Vec2::new(0.0, JUMP_IMPULSE_VALUE);
        }
    }
}
