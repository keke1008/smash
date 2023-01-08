mod action;
mod animation;
mod assets;
mod constants;
mod event;
mod state;

pub(in crate::game) use assets::PlayerAssets;
use iyes_loopless::prelude::*;

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{game::tags::Player, state::AppState};

use self::{
    constants::{COLLIDER_HALF_HEIGHT, COLLIDER_RADIUS, INITIAL_TRANSLATION},
    state::CurrentPlayerState,
};

#[derive(Debug)]
pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(state::PlayerStatePlugin)
            .add_plugin(event::PlayerEventPlugin)
            .add_plugin(action::PlayerActionPlugin)
            .add_plugin(animation::PlayerAnimationPlugin)
            .add_enter_system(AppState::InGame, spawn_player);
    }
}

fn spawn_player(mut commands: Commands, assets: Res<PlayerAssets>) {
    commands
        .spawn((Player, Name::new("Player")))
        .insert(CurrentPlayerState::default())
        .insert((
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::cylinder(COLLIDER_HALF_HEIGHT, COLLIDER_RADIUS),
            KinematicCharacterController::default(),
            SpatialBundle::from_transform(Transform::from_translation(INITIAL_TRANSLATION)),
        ))
        .with_children(|builder| {
            builder.spawn(SceneBundle {
                scene: assets.scene.clone(),
                transform: Transform::from_xyz(0.0, -COLLIDER_HALF_HEIGHT, 0.0)
                    .with_rotation(Quat::from_rotation_y(PI)),
                ..default()
            });
        });
}
