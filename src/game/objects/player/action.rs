use bevy::prelude::*;
use bevy_rapier3d::prelude::KinematicCharacterController;
use iyes_loopless::prelude::*;

use crate::{
    game::{
        objects::{player::constants::PUNCH_X_OFFSET, punch::SpawnPunch},
        tags::Player,
    },
    state::AppState,
};

use super::{
    constants::MOVEMENT_PER_SEC,
    state::{CurrentPlayerState, PlayerState},
};

#[derive(Debug)]
pub(super) struct PlayerActionPlugin;

impl Plugin for PlayerActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_action.run_in_state(AppState::InGame));
    }
}

fn apply_action(
    mut commands: Commands,
    mut player: Query<
        (
            &CurrentPlayerState,
            &Transform,
            &mut KinematicCharacterController,
        ),
        With<Player>,
    >,
    mut previous_state: Local<PlayerState>,
    time: Res<Time>,
) {
    let Ok((current_state, transform, mut controller)) = player.get_single_mut() else {
        return;
    };

    use PlayerState::*;
    match current_state.get() {
        Run => {
            let translation = transform.forward() * MOVEMENT_PER_SEC * time.delta_seconds();
            controller.translation = Some(translation);
        }
        state @ (LeftPunch | RightPunch) if *previous_state != state => {
            let diff = match state {
                LeftPunch => transform.left() * PUNCH_X_OFFSET,
                RightPunch => transform.right() * PUNCH_X_OFFSET,
                _ => unreachable!(),
            };
            commands.add(SpawnPunch {
                transform: transform.with_translation(transform.translation + diff),
            });
        }
        _ => {}
    }

    *previous_state = current_state.get();
}
