use bevy::prelude::*;

use crate::game::constants::{STAGE_DEPTH, STAGE_WIDTH};

pub(super) const HALF_WIDTH: f32 = STAGE_WIDTH / 2.0;
pub(super) const HALF_DEPTH: f32 = STAGE_DEPTH / 2.0;
pub(super) const HALF_THICKNESS: f32 = 0.1;
pub(super) const COLOR: Color = Color::rgb(0.5, 0.9, 0.5);
pub(super) const FRICTION: f32 = 0.8;
