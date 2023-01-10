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
        app.add_enter_system(AppState::InGame, setup)
            .add_exit_system(AppState::InGame, despawn)
            .add_system(update_event.run_in_state(AppState::InGame).label("update"))
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .after("update")
                    .with_system(send_movement_event)
                    .with_system(send_attack_event)
                    .with_system(send_rotation_event)
                    .into(),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(EventTransition::<Option<PlayerMovementEvent>>::default());
    commands.insert_resource(EventTransition::<Option<PlayerAttackEvent>>::default());
    commands.insert_resource(EventTransition::<Option<PlayerRotationEvent>>::default());
}

fn despawn(mut commands: Commands) {
    commands.remove_resource::<EventTransition<Option<PlayerMovementEvent>>>();
    commands.remove_resource::<EventTransition<Option<PlayerAttackEvent>>>();
    commands.remove_resource::<EventTransition<Option<PlayerRotationEvent>>>();
}

#[derive(Resource, Debug, Default)]
struct EventTransition<E> {
    current: E,
    previous: Option<E>,
}

impl<E: PartialEq + Copy> EventTransition<E> {
    fn push(&mut self, event: E) {
        if self.current == event {
            return;
        }
        let previous = std::mem::replace(&mut self.current, event);
        self.previous = Some(previous);
    }

    fn take_change(&mut self) -> Option<E> {
        self.previous.take()
    }

    fn current(&self) -> E {
        self.current
    }
}

fn update_event(
    mut movement: ResMut<EventTransition<Option<PlayerMovementEvent>>>,
    mut attack: ResMut<EventTransition<Option<PlayerAttackEvent>>>,
    mut rotation: ResMut<EventTransition<Option<PlayerRotationEvent>>>,
) {
    if let Some(movement_prediction) = js::get_movement_prediction() {
        use MovementPrediction::*;
        let (movement_event, attack_event) = match movement_prediction {
            Stay => (None, None),
            Run => (Some(PlayerMovementEvent::MoveForward), None),
            LeftPunch => (None, Some(PlayerAttackEvent::LeftPunch)),
            RightPunch => (None, Some(PlayerAttackEvent::RightPunch)),
        };
        movement.push(movement_event);
        attack.push(attack_event);
    }

    if let Some(rotation_prediction) = js::get_rotation_prediction() {
        info!("{rotation_prediction:?}");
        use RotationPrediction::*;
        let rotation_event = match rotation_prediction {
            Stay => None,
            RotateLeft => Some(PlayerRotationEvent::Left),
            RotateRight => Some(PlayerRotationEvent::Right),
        };
        rotation.push(rotation_event);
    }
}

fn send_movement_event(
    mut movement: ResMut<EventTransition<Option<PlayerMovementEvent>>>,
    mut movement_tx: EventWriter<PlayerMovementEvent>,
) {
    let event = if let Some(Some(event)) = movement.take_change() {
        event
    } else if movement.current() == Some(PlayerMovementEvent::MoveForward) {
        PlayerMovementEvent::MoveForward
    } else {
        return;
    };
    movement_tx.send(event);
}

fn send_attack_event(
    mut attack: ResMut<EventTransition<Option<PlayerAttackEvent>>>,
    mut attack_tx: EventWriter<PlayerAttackEvent>,
) {
    if let Some(Some(event)) = attack.take_change() {
        attack_tx.send(event);
    }
}

fn send_rotation_event(
    rotation: ResMut<EventTransition<Option<PlayerRotationEvent>>>,
    mut rotation_tx: EventWriter<PlayerRotationEvent>,
) {
    if let Some(event) = rotation.current() {
        rotation_tx.send(event);
    }
}
