mod assets;
mod constants;

pub(in crate::game) use assets::CubeAssets;

use bevy::{ecs::system::Command, prelude::*};
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;

use crate::state::AppState;

use self::constants::{
    DENSITY_METAL, DENSITY_STONE, DENSITY_WOODEN, FRICTION, HALF_SIZE, MIN_HEIGHT, MODEL_SCALE,
};

#[derive(Debug)]
pub(super) struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(kill.run_in_state(AppState::InGame));
    }
}

#[derive(Debug)]
pub(super) struct SpawnCube {
    pub(super) cube_type: CubeType,
    pub(super) position: Vec3,
}

impl Command for SpawnCube {
    fn write(self, world: &mut World) {
        world.resource_scope(|world, assets: Mut<CubeAssets>| {
            let scene_handle = self.cube_type.get_asset(&assets);
            world
                .spawn((Cube, Name::new("Cube")))
                .insert((
                    RigidBody::Dynamic,
                    Collider::cuboid(HALF_SIZE, HALF_SIZE, HALF_SIZE),
                    ColliderMassProperties::Density(self.cube_type.density()),
                    Friction::new(FRICTION),
                    SpatialBundle::from_transform(Transform::from_translation(self.position)),
                ))
                .with_children(|builder| {
                    builder.spawn(SceneBundle {
                        scene: scene_handle,
                        transform: Transform::from_scale(Vec3::splat(MODEL_SCALE)),

                        ..default()
                    });
                });
        });
    }
}

#[derive(Debug)]
pub(super) enum CubeType {
    Wooden,
    Stone,
    Metal,
}

impl CubeType {
    fn get_asset(&self, assets: &CubeAssets) -> Handle<Scene> {
        use CubeType::*;
        let handle = match self {
            Wooden => &assets.wooden,
            Stone => &assets.stone,
            Metal => &assets.metal,
        };
        handle.clone_weak()
    }

    fn density(&self) -> f32 {
        use CubeType::*;
        match self {
            Wooden => DENSITY_WOODEN,
            Stone => DENSITY_STONE,
            Metal => DENSITY_METAL,
        }
    }
}

#[derive(Component, Debug)]
struct Cube;

fn kill(mut commands: Commands, cubes: Query<(Entity, &Transform), With<Cube>>) {
    for (cube_entity, transform) in cubes.iter() {
        if transform.translation.y < MIN_HEIGHT {
            commands.entity(cube_entity).despawn_recursive();
        }
    }
}
