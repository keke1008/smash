mod constants;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{game::tags::Player, state::AppState, utils::despawn_recursive};

use self::constants::{OFFSET_Y, OFFSET_Z};

#[derive(Debug)]
pub(super) struct CameraPlungin;

impl Plugin for CameraPlungin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, spawn_camera)
            .add_exit_system(AppState::InGame, despawn_recursive::<Camera>)
            .add_system(follow_player.run_in_state(AppState::InGame));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn((Camera, Name::new("Camera")))
        .insert(Camera3dBundle::default());
}

#[derive(Component, Debug)]
struct Camera;

fn follow_player(
    mut query: ParamSet<(
        Query<&Transform, (With<Player>, Changed<Transform>)>,
        Query<&mut Transform, With<Camera>>,
    )>,
) {
    let player = query.p0();
    let Ok(player) = player.get_single() else {
        return;
    };
    let offset = player.up() * OFFSET_Y + player.back() * OFFSET_Z;
    let camera_translation = player.translation + offset;
    let player_translation = player.translation;

    let mut camera = query.p1();
    let mut camera = camera.single_mut();
    camera.translation = camera_translation;
    camera.look_at(player_translation, Vec3::Y);
}
