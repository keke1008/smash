mod constants;

use bevy::{ecs::system::Command, prelude::*};
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{game::groups, state::AppState, utils::despawn_recursive};

use self::constants::{COLLIDER_RADIUS, DENSITY, MOVEMENT_PER_SEC, RANGE};

#[derive(Debug)]
pub(super) struct PunchPlugin;

impl Plugin for PunchPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update.run_in_state(AppState::InGame))
            .add_exit_system(AppState::InGame, despawn_recursive::<Punch>);
    }
}

#[derive(Debug)]
pub(super) struct SpawnPunch {
    pub(super) transform: Transform,
}

impl Command for SpawnPunch {
    fn write(self, world: &mut World) {
        const GROUPS: CollisionGroups =
            CollisionGroups::new(groups::PUNCH, groups::ALL.difference(groups::PLAYER));
        world
            .spawn((Punch, Name::new("Punch")))
            .insert(Distance(0.0))
            .insert((
                RigidBody::KinematicPositionBased,
                TransformBundle::from_transform(self.transform),
                Collider::ball(COLLIDER_RADIUS),
                ColliderMassProperties::Density(DENSITY),
                GROUPS,
                KinematicCharacterController {
                    filter_groups: Some(GROUPS.into()),
                    ..default()
                },
            ));
    }
}

#[derive(Debug, Component)]
struct Punch;

#[derive(Component, Debug)]
struct Distance(f32);

fn update(
    mut commands: Commands,
    mut punches: Query<
        (
            Entity,
            &Transform,
            &mut Distance,
            &mut KinematicCharacterController,
        ),
        With<Punch>,
    >,
    time: Res<Time>,
) {
    for (punch_entity, transform, mut distance, mut controller) in punches.iter_mut() {
        if distance.0 > RANGE {
            commands.entity(punch_entity).despawn_recursive();
            continue;
        }

        let forward = transform.forward() * MOVEMENT_PER_SEC * time.delta_seconds();
        controller.translation = Some(forward);
        distance.0 += forward.length();
    }
}
