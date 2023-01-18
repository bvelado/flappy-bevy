use bevy_rapier2d::prelude::Group;

pub const GAME_WIDTH: f32 = 576.0;
pub const GAME_HEIGHT: f32 = 324.0;

pub const COLLISION_GROUP_GAME_OVER: Group = Group::GROUP_2;
pub const COLLISION_GROUP_PLAYER: Group = Group::GROUP_3;
pub const COLLISION_GROUP_OPENING: Group = Group::GROUP_4;

pub const GRAVITY: f32 = -1080.0;
pub const JUMP_IMPULSE_VALUE: f32 = 300.0;

pub const BASE_GAME_SPEED: f32 = 1.1;
pub const BASE_MOVE_SPEED: f32 = 94.0;
pub const ACCELERATION_FACTOR: f32 = 1.028;
pub const SECONDS_BETWEEN_ACCELERATION_TICK: f32 = 3.6;