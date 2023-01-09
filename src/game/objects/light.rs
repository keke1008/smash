mod constants;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::state::AppState;

use self::constants::{BRIGHTNESS, COLOR, ILLUMINANCE, LOOKING_AT, SHADOWS_ENABLED, TRANSLATION};

#[derive(Debug)]
pub(super) struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, spawn_light);
    }
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(TRANSLATION).looking_at(LOOKING_AT, Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: ILLUMINANCE,
            shadows_enabled: SHADOWS_ENABLED,
            ..default()
        },
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: COLOR,
        brightness: BRIGHTNESS,
    });
}
