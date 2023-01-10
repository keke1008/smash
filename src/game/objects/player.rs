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

use crate::{
    game::{
        groups,
        objects::player::constants::{DENSITY, FRICTION},
        tags::Player,
    },
    state::AppState,
    utils::despawn_recursive,
};

use self::{
    constants::{COLLIDER_HALF_HEIGHT, COLLIDER_RADIUS, INITIAL_TRANSLATION, MIN_HEIGHT},
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
            .add_enter_system(AppState::InGame, spawn_player)
            .add_exit_system(AppState::InGame, despawn_recursive::<Player>)
            .add_system(despawn.run_in_state(AppState::InGame));
    }
}

fn spawn_player(mut commands: Commands, assets: Res<PlayerAssets>) {
    const GROUPS: CollisionGroups = CollisionGroups::new(groups::PLAYER, groups::ALL);

    commands
        .spawn((Player, Name::new("Player")))
        .insert(CurrentPlayerState::default())
        .insert((
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::cylinder(COLLIDER_HALF_HEIGHT, COLLIDER_RADIUS),
            ColliderMassProperties::Density(DENSITY),
            Friction {
                coefficient: FRICTION,
                combine_rule: CoefficientCombineRule::Max,
            },
            GROUPS,
            KinematicCharacterController {
                filter_groups: Some(GROUPS.into()),
                custom_mass: Some(DENSITY),
                slide: false,
                ..default()
            },
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

fn despawn(mut commands: Commands, player: Query<(Entity, &Transform), With<Player>>) {
    let Ok((player_entity, transform)) = player.get_single() else {
        return;
    };

    if transform.translation.y < MIN_HEIGHT {
        commands.entity(player_entity).despawn_recursive();
        commands.insert_resource(NextState(crate::game::state::PlayerState::Dead));
    }
}
