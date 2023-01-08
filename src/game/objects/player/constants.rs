use std::f32::consts::PI;

use bevy::prelude::*;

pub(super) const INITIAL_TRANSLATION: Vec3 = Vec3::new(0.0, 1.0, 0.0);
pub(super) const COLLIDER_RADIUS: f32 = 0.3;
pub(super) const COLLIDER_HALF_HEIGHT: f32 = 0.8;
pub(super) const ROTATION_PER_SEC: f32 = PI / 2.0;
pub(super) const MOVEMENT_PER_SEC: f32 = 4.0;
pub(super) const PUNCH_X_OFFSET: f32 = 0.2;
