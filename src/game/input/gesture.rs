mod js;
mod predictions;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::state::AppState;

use self::predictions::{MovementPrediction, RotationPrediction};
use super::{PlayerAttackEvent, PlayerMovementEvent, PlayerRotationEvent};

#[derive(Debug)]
pub(super) struct GestureInputPlugin;

impl Plugin for GestureInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_predictions.run_in_state(AppState::InGame));
    }
}

fn handle_predictions(
    mut previous_movement_event: Local<Option<PlayerMovementEvent>>,
    mut previpus_attack_event: Local<Option<PlayerAttackEvent>>,
    mut previous_rotation_event: Local<Option<PlayerRotationEvent>>,
    mut movement_tx: EventWriter<PlayerMovementEvent>,
    mut attack_tx: EventWriter<PlayerAttackEvent>,
    mut rotation_tx: EventWriter<PlayerRotationEvent>,
) {
    if let Some(movement_prediction) = js::get_movement_prediction() {
        use MovementPrediction::*;
        let (movement_event, attack_event) = match movement_prediction {
            Stay => (None, None),
            Run => (Some(PlayerMovementEvent::MoveForward), None),
            LeftPunch => (None, Some(PlayerAttackEvent::LeftPunch)),
            RightPunch => (None, Some(PlayerAttackEvent::RightPunch)),
        };

        if movement_event != *previous_movement_event || movement_event.is_some() {
            *previous_movement_event = movement_event.clone();
            if let Some(event) = movement_event {
                movement_tx.send(event);
            }
        }

        if attack_event != *previpus_attack_event {
            *previpus_attack_event = attack_event.clone();
            if let Some(event) = attack_event {
                attack_tx.send(event);
            }
        }
    } else if let Some(event) = previous_movement_event.clone() {
        movement_tx.send(event);
    }

    let event = if let Some(rotation_prediction) = js::get_rotation_prediction() {
        use RotationPrediction::*;
        match rotation_prediction {
            Stay => None,
            RotateLeft => Some(PlayerRotationEvent::Left),
            RotateRight => Some(PlayerRotationEvent::Right),
        }
    } else {
        previous_rotation_event.clone()
    };
    *previous_rotation_event = event.clone();
    if let Some(event) = event {
        rotation_tx.send(event);
    }
}
