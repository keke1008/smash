use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    game::{
        input::{PlayerAttackEvent, PlayerMovementEvent, PlayerRotationEvent},
        tags::Player,
    },
    state::AppState,
};

use super::{
    constants::ROTATION_PER_SEC,
    state::{CurrentPlayerState, PlayerState},
};

#[derive(Debug)]
pub(super) struct PlayerEventPlugin;

impl Plugin for PlayerEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .with_system(movement)
                .with_system(rotation)
                .with_system(attack)
                .into(),
        );
    }
}

fn movement(
    mut player: Query<&mut CurrentPlayerState, With<Player>>,
    mut events: EventReader<PlayerMovementEvent>,
) {
    let Ok(mut current_state) = player.get_single_mut() else {
        return;
    };

    let state = match events.iter().last() {
        Some(PlayerMovementEvent::MoveForward) => PlayerState::Run,
        None => PlayerState::Stay,
    };
    current_state.set(state);
}

fn rotation(
    mut events: EventReader<PlayerRotationEvent>,
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut player_transform) = player.get_single_mut() else {
        return;
    };

    let rotation_y = match events.iter().last() {
        Some(PlayerRotationEvent::Left) => ROTATION_PER_SEC,
        Some(PlayerRotationEvent::Right) => -ROTATION_PER_SEC,
        None => return,
    };
    player_transform.rotate_y(rotation_y * time.delta_seconds());
}

fn attack(
    mut player: Query<&mut CurrentPlayerState, With<Player>>,
    mut events: EventReader<PlayerAttackEvent>,
) {
    let Ok(mut current_state) = player.get_single_mut() else {
        return;
    };

    let state = match events.iter().last() {
        Some(PlayerAttackEvent::LeftPunch) => PlayerState::LeftPunch,
        Some(PlayerAttackEvent::RightPunch) => PlayerState::RightPunch,
        None => return,
    };
    current_state.set(state);
}
