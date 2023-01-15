use bevy::{
    prelude::{
        Commands, Component, Entity, Handle, Image, Query, Res, ResMut, Resource, Transform, Vec2,
        Vec3, With,
    },
    sprite::{Anchor, Sprite, SpriteBundle},
    time::Time,
};
use bevy_rapier2d::prelude::{Collider, CollisionGroups, GravityScale, RigidBody};
use bevy_turborand::{DelegatedRng, GlobalRng};

use crate::{
    assets::GameAssets,
    consts::{
        BASE_MOVE_SPEED, COLLISION_GROUP_DEATH, COLLISION_GROUP_PLAYER, GAME_HEIGHT, GAME_WIDTH,
    },
    game::{GameSpeed, HorizontalMove},
};

const OBSTACLE_SPRITE_WIDTH: f32 = 18.0;
const OBSTACLE_SPRITE_HEIGHT: f32 = 252.0;

const OBSTACLE_OPENING_MIN_BOTTOM_Y_OFFSET: f32 = 54.0;
const OBSTACLE_OPENING_MAX_TOP_Y_OFFSET: f32 = 36.0;

const OBSTACLE_GAP_MIN_HORIZONTAL_DISTANCE: f32 = 90.0;
const OBSTACLE_GAP_MAX_HORIZONTAL_DISTANCE: f32 = 130.0;

const OBSTACLE_OPENING_HEIGHT: f32 = 80.0;

#[derive(Default, Resource)]
pub struct ObstaclesData {
    traveled_distance: f32,
    last_obstacle_distance: f32,
}

#[derive(Component)]
pub struct Obstacle;

pub fn update_obstacles_data(
    mut obstacle_data: ResMut<ObstaclesData>,
    game_speed: Res<GameSpeed>,
    time: Res<Time>,
) {
    let frame_traveled_distance = BASE_MOVE_SPEED * game_speed.factor * time.delta_seconds();
    obstacle_data.traveled_distance += frame_traveled_distance;
    obstacle_data.last_obstacle_distance += frame_traveled_distance;
}

pub fn spawn_obstacles(
    mut commands: Commands,
    mut global_rng: ResMut<GlobalRng>,
    mut obstacles_data: ResMut<ObstaclesData>,
    game_assets: Res<GameAssets>,
) {
    if obstacles_data.last_obstacle_distance > OBSTACLE_GAP_MIN_HORIZONTAL_DISTANCE {
        let screen_offset_x = GAME_WIDTH / 2.0;
        let gap_offset_x = random_gap(&mut global_rng);
        let obstacle_pos_x = screen_offset_x + gap_offset_x;

        let obstacle_opening_bottom_y_pos = random_opening_position_bottom_y(&mut global_rng);

        spawn_obstacle_entities(
            &mut commands,
            game_assets.obstacle_image.clone(),
            obstacle_pos_x,
            obstacle_opening_bottom_y_pos,
        );

        obstacles_data.last_obstacle_distance = obstacles_data.last_obstacle_distance
            - OBSTACLE_GAP_MIN_HORIZONTAL_DISTANCE
            - gap_offset_x;
    }
}

pub fn despawn_passed_obstacles(
    mut commands: Commands,
    q_obstacles: Query<(Entity, &Transform), With<Obstacle>>,
) {
    for (e, t) in q_obstacles.iter() {
        // should be obstacle sprite width / 2 but the delta is used as a safety measure
        // to despawn only off screen obstacles
        if t.translation.x < -OBSTACLE_SPRITE_WIDTH - GAME_WIDTH / 2.0 {
            commands.entity(e).despawn();
        }
    }
}

pub fn reset_obstacles_state(
    mut commands: Commands,
    q_obstacles: Query<Entity, With<Obstacle>>,
    mut obstacles_data: ResMut<ObstaclesData>,
) {
    for e in q_obstacles.iter() {
        commands.entity(e).despawn();
    }
    *obstacles_data = ObstaclesData::default();
}

fn spawn_obstacle_entities(
    commands: &mut Commands,
    obstacle_image_handle: Handle<Image>,
    obstacle_pos_x: f32,
    obstacle_opening_bottom_y_pos: f32,
) {
    let sprite_height_offset_y = OBSTACLE_SPRITE_HEIGHT / 2.0;
    spawn_obstacle_entity(
        commands,
        obstacle_image_handle.clone(),
        Vec2::new(
            obstacle_pos_x,
            obstacle_opening_bottom_y_pos - sprite_height_offset_y,
        ),
        false,
    );
    spawn_obstacle_entity(
        commands,
        obstacle_image_handle,
        Vec2::new(
            obstacle_pos_x,
            obstacle_opening_bottom_y_pos + sprite_height_offset_y + OBSTACLE_OPENING_HEIGHT,
        ),
        true,
    );
}

fn random_opening_position_bottom_y(global_rng: &mut ResMut<GlobalRng>) -> f32 {
    map_range(
        global_rng.f32(),
        0.0,
        1.0,
        -GAME_HEIGHT / 2.0 + OBSTACLE_OPENING_MIN_BOTTOM_Y_OFFSET,
        GAME_HEIGHT / 2.0 - OBSTACLE_OPENING_MAX_TOP_Y_OFFSET - OBSTACLE_OPENING_HEIGHT,
    )
}

fn random_gap(global_rng: &mut ResMut<GlobalRng>) -> f32 {
    let gap_range = OBSTACLE_GAP_MAX_HORIZONTAL_DISTANCE - OBSTACLE_GAP_MIN_HORIZONTAL_DISTANCE;
    global_rng.f32() * gap_range + OBSTACLE_GAP_MIN_HORIZONTAL_DISTANCE
}

fn map_range(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

fn spawn_obstacle_entity(
    commands: &mut Commands,
    obstacle_image_handle: Handle<Image>,
    obstacle_pos: Vec2,
    flip_y: bool,
) {
    commands.spawn((
        SpriteBundle {
            texture: obstacle_image_handle.clone(),
            sprite: Sprite {
                flip_y,
                anchor: Anchor::Center,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(obstacle_pos.x, obstacle_pos.y, 0.0)),
            ..Default::default()
        },
        HorizontalMove { factor: 1.0 },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(9.0, OBSTACLE_SPRITE_HEIGHT / 2.0),
        GravityScale(0.0),
        CollisionGroups::new(COLLISION_GROUP_DEATH, COLLISION_GROUP_PLAYER),
        Obstacle,
    ));
}
