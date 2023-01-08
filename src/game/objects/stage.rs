use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use constants::{COLOR, HALF_DEPTH, HALF_THICKNESS, HALF_WIDTH};
use iyes_loopless::prelude::*;

use crate::state::AppState;

use self::constants::FRICTION;

mod constants;

#[derive(Debug)]
pub(super) struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, spawn_stage);
    }
}

fn spawn_stage(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Box::new(
        HALF_WIDTH * 2.0,
        HALF_THICKNESS * 2.0,
        HALF_DEPTH * 2.0,
    )));
    let material = materials.add(StandardMaterial {
        base_color: COLOR,
        ..default()
    });

    commands
        .spawn(Name::new("Ground"))
        .insert((
            RigidBody::Fixed,
            Collider::cuboid(HALF_WIDTH, HALF_THICKNESS, HALF_DEPTH),
            Friction::new(FRICTION),
        ))
        .insert(PbrBundle {
            mesh,
            material,
            ..default()
        });
}
