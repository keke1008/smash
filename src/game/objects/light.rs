mod constants;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{state::AppState, utils::despawn_recursive};

use self::constants::{BRIGHTNESS, COLOR, ILLUMINANCE, LOOKING_AT, SHADOWS_ENABLED, TRANSLATION};

#[derive(Debug)]
pub(super) struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, spawn_light)
            .add_exit_system_set(
                AppState::InGame,
                ConditionSet::new()
                    .with_system(despawn_recursive::<Light>)
                    .with_system(remove_resource)
                    .into(),
            );
    }
}

#[derive(Component, Debug)]
struct Light;

fn spawn_light(mut commands: Commands) {
    commands
        .spawn((Light, Name::new("DirectionalLight")))
        .insert(DirectionalLightBundle {
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

fn remove_resource(mut commands: Commands) {
    commands.remove_resource::<AmbientLight>();
}
