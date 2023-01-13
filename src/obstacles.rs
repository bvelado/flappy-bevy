use bevy::{
    prelude::{info, Commands, Res, ResMut, Resource, Transform, Vec3},
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

const OBSTACLE_SPRITE_HEIGHT: f32 = 252.0;

const OBSTACLE_MIN_HEIGHT: f32 = 54.0;
const OBSTACLE_MAX_HEIGHT: f32 = 234.0;
const OBSTACLE_MIN_GAP: f32 = 90.0;
const OBSTACLE_MAX_GAP: f32 = 130.0;

#[derive(Default, Resource)]
pub struct ObstaclesData {
    traveled_distance: f32,
    last_obstacle_distance: f32,
}

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
    if obstacles_data.last_obstacle_distance > OBSTACLE_MIN_GAP {
        let screen_offset_x = GAME_WIDTH / 2.0;
        let gap_offset_x = random_gap(&mut global_rng);
        let obstacle_pos_x = screen_offset_x + gap_offset_x;

        let obstacle_height = random_height(&mut global_rng);
        let sprite_height_offset_y = OBSTACLE_SPRITE_HEIGHT / 2.0;
        let obstacle_pos_y = obstacle_height - sprite_height_offset_y - GAME_HEIGHT / 2.0;

        commands.spawn((
            SpriteBundle {
                texture: game_assets.obstacle_image.clone(),
                sprite: Sprite {
                    anchor: Anchor::Center,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(
                    obstacle_pos_x,
                    obstacle_pos_y,
                    0.0,
                )),
                ..Default::default()
            },
            HorizontalMove { factor: 1.0 },
            RigidBody::KinematicPositionBased,
            Collider::cuboid(9.0, OBSTACLE_SPRITE_HEIGHT / 2.0),
            GravityScale(0.0),
            CollisionGroups::new(COLLISION_GROUP_DEATH, COLLISION_GROUP_PLAYER),
        ));

        obstacles_data.last_obstacle_distance =
            obstacles_data.last_obstacle_distance - OBSTACLE_MIN_GAP - gap_offset_x;
        info!(
            "Spawned an obstacle at {}. Last obstacle distance is now {}.",
            obstacle_pos_x, &obstacles_data.last_obstacle_distance
        );
    }
}

fn random_height(global_rng: &mut ResMut<GlobalRng>) -> f32 {
    let height_range = OBSTACLE_MAX_HEIGHT - OBSTACLE_MIN_HEIGHT;
    global_rng.f32_normalized() * height_range + OBSTACLE_MIN_HEIGHT
}

fn random_gap(global_rng: &mut ResMut<GlobalRng>) -> f32 {
    let gap_range = OBSTACLE_MAX_GAP - OBSTACLE_MIN_GAP;
    global_rng.f32_normalized() * gap_range + OBSTACLE_MIN_GAP
}
