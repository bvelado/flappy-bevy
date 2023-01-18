use bevy::{
    prelude::{Commands, Component, Res, Transform, Vec3},
    sprite::{Anchor, Sprite, SpriteBundle},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{Collider, CollisionGroups, GravityScale, LockedAxes, RigidBody};

use crate::{
    assets::GameAssets,
    consts::{COLLISION_GROUP_GAME_OVER, COLLISION_GROUP_PLAYER, GAME_HEIGHT, GAME_WIDTH},
    game::HorizontalMove,
    obstacles::Obstacle,
};

#[derive(Component)]
pub struct Ground;

// pub fn spawn_world_background(mut commands: Commands, game_assets: Res<GameAssets>) {
//     commands.spawn(SpriteBundle {
//         texture: game_assets.background_image.clone(),
//         sprite: Sprite {
//             anchor: Anchor::Center,
//             ..Default::default()
//         },
//         transform: Transform::from_translation(Vec3::ZERO),
//         ..Default::default()
//     });
// }

pub fn spawn_world_ground(mut commands: Commands, game_assets: Res<GameAssets>) {
    // spawn both backgrounds
    commands.spawn((
        SpriteBundle {
            texture: game_assets.ground_image.clone(),
            sprite: Sprite {
                anchor: Anchor::CenterRight,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(GAME_WIDTH / 2., 0., 0.)),
            ..Default::default()
        },
        HorizontalMove { factor: 1.0 },
        Ground,
    ));
    commands.spawn((
        SpriteBundle {
            texture: game_assets.ground_image.clone(),
            sprite: Sprite {
                anchor: Anchor::CenterRight,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(GAME_WIDTH + GAME_WIDTH / 2., 0., 0.)),
            ..Default::default()
        },
        HorizontalMove { factor: 1.0 },
        Ground,
    ));

    let y_world_offset = GAME_HEIGHT / 2.0;
    let collider_height = 200.0;
    let y_collider_offset = collider_height / 2.0 + y_world_offset;
    let bottom_collider_tiles_offset = 18.0 * 2.0; // 2 tiles of 18px each

    // spawn static top and down death colliders
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, y_collider_offset, 0.0)),
        Collider::cuboid(GAME_WIDTH / 2.0, collider_height / 2.0),
        RigidBody::Fixed,
        GravityScale(0.0),
        LockedAxes::TRANSLATION_LOCKED | LockedAxes::ROTATION_LOCKED,
        CollisionGroups::new(COLLISION_GROUP_GAME_OVER, COLLISION_GROUP_PLAYER),
        Obstacle::GameOverStatic,
    ));
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(
            0.0,
            -y_collider_offset + bottom_collider_tiles_offset,
            0.0,
        )),
        Collider::cuboid(GAME_WIDTH / 2.0, collider_height / 2.0),
        RigidBody::Fixed,
        GravityScale(0.0),
        LockedAxes::TRANSLATION_LOCKED | LockedAxes::ROTATION_LOCKED,
        CollisionGroups::new(COLLISION_GROUP_GAME_OVER, COLLISION_GROUP_PLAYER),
        Obstacle::GameOverStatic,
    ));
}
