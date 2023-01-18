use bevy::{
    prelude::{
        Assets, Commands, Component, EventReader, Input, KeyCode, MouseButton, Quat, Query, Res,
        ResMut, Resource, Transform, Vec2, Vec3, With,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Timer, TimerMode},
};
use bevy_rapier2d::prelude::{
    ActiveEvents, Ccd, Collider, CollisionGroups, GravityScale, LockedAxes, RigidBody, Velocity,
};

use crate::{
    animations::{AnimationIndices, AnimationTimer},
    consts::{COLLISION_GROUP_GAME_OVER, COLLISION_GROUP_OPENING, COLLISION_GROUP_PLAYER},
    events::GameEvent,
};
use crate::{assets::GameAssets, consts::JUMP_IMPULSE_VALUE};

#[derive(Component)]
pub struct Player;

#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct PlayerScore {
    pub value: u16,
}

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
        GravityScale(0.0),
        CollisionGroups::new(
            COLLISION_GROUP_PLAYER,
            COLLISION_GROUP_GAME_OVER.union(COLLISION_GROUP_OPENING),
        ),
        Velocity::zero(),
        LockedAxes::from(LockedAxes::TRANSLATION_LOCKED_X | LockedAxes::ROTATION_LOCKED),
        ActiveEvents::COLLISION_EVENTS,
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

pub fn enable_player_gravity(mut q_player: Query<&mut GravityScale, With<Player>>) {
    q_player.single_mut().0 = 1.0;
}

pub fn reset_player_state(
    mut q_player: Query<(&mut GravityScale, &mut Velocity, &mut Transform), With<Player>>,
) {
    if let Ok((mut gs, mut v, mut t)) = q_player.get_single_mut() {
        gs.0 = 0.0;
        t.translation = Vec3::ZERO;
        t.rotation = Quat::IDENTITY;
        v.linvel = Vec2::ZERO;
        v.angvel = 0.0;
    }
}

pub fn reset_player_score(mut player_score: ResMut<PlayerScore>) {
    player_score.value = 0;
}

pub fn handle_game_event_player_passed_opening(
    mut ev_game: EventReader<GameEvent>,
    mut player_score: ResMut<PlayerScore>,
) {
    for _ in ev_game
        .iter()
        .filter(|&ev| ev == &GameEvent::PlayerPassedAnOpening)
    {
        player_score.value += 1;
    }
}
