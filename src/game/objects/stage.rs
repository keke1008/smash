use bevy::{ecs::system::Command, prelude::*};
use bevy_rapier3d::prelude::*;
use constants::{COLOR, HALF_DEPTH, HALF_THICKNESS, HALF_WIDTH};

mod constants;

#[derive(Debug)]
pub(super) struct SpawnStage;

impl Command for SpawnStage {
    fn write(self, world: &mut World) {
        let mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Mesh::from(shape::Box::new(
                HALF_WIDTH * 2.0,
                HALF_THICKNESS * 2.0,
                HALF_DEPTH * 2.0,
            )));
        let material = world
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial {
                base_color: COLOR,
                ..default()
            });

        world
            .spawn(Name::new("Ground"))
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(HALF_WIDTH, HALF_THICKNESS, HALF_DEPTH))
            .insert(PbrBundle {
                mesh,
                material,
                ..default()
            });
    }
}
