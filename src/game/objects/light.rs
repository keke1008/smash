mod constants;

use bevy::{ecs::system::Command, prelude::*};

use self::constants::{ILLUMINANCE, LOOKING_AT, SHADOWS_ENABLED, TRANSLATION};

#[derive(Debug)]
pub(super) struct SpawnLight;

impl Command for SpawnLight {
    fn write(self, world: &mut World) {
        world.spawn(DirectionalLightBundle {
            transform: Transform::from_translation(TRANSLATION).looking_at(LOOKING_AT, Vec3::Y),
            directional_light: DirectionalLight {
                illuminance: ILLUMINANCE,
                shadows_enabled: SHADOWS_ENABLED,
                ..default()
            },
            ..default()
        });
    }
}
