//! Debugging input device
use bevy::{prelude::*, utils::HashMap};
use iyes_loopless::prelude::*;

use crate::state::AppState;

use super::{PlayerAttackEvent, PlayerMovementEvent, PlayerRotationEvent};

#[derive(Debug)]
pub(super) struct KeyboardInputPlugin;

impl Plugin for KeyboardInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, setup)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(tick)
                    .with_system(handle_keyboard_events)
                    .into(),
            );
    }
}

const MOVE_FORWARD: KeyCode = KeyCode::W;
const ROTATE_LEFT: KeyCode = KeyCode::H;
const ROTATE_RIGHT: KeyCode = KeyCode::L;
const LEFT_PUNCH: KeyCode = KeyCode::A;
const RIGHT_PUNCH: KeyCode = KeyCode::D;

fn setup(mut commands: Commands) {
    commands.init_resource::<InputInterval>();
}
fn tick(mut interval: ResMut<InputInterval>, time: Res<Time>) {
    interval.tick(time);
}

fn handle_keyboard_events(
    key_state: Res<Input<KeyCode>>,
    mut interval: ResMut<InputInterval>,
    mut movement_tx: EventWriter<PlayerMovementEvent>,
    mut rotation_tx: EventWriter<PlayerRotationEvent>,
    mut attack_tx: EventWriter<PlayerAttackEvent>,
) {
    if key_state.pressed(MOVE_FORWARD) {
        movement_tx.send(PlayerMovementEvent::MoveForward);
    }
    if key_state.pressed(ROTATE_LEFT) {
        rotation_tx.send(PlayerRotationEvent::Left);
    }
    if key_state.pressed(ROTATE_RIGHT) {
        rotation_tx.send(PlayerRotationEvent::Right);
    }
    if key_state.pressed(LEFT_PUNCH) && interval.check(LEFT_PUNCH) {
        attack_tx.send(PlayerAttackEvent::LeftPunch);
    }
    if key_state.pressed(RIGHT_PUNCH) && interval.check(RIGHT_PUNCH) {
        attack_tx.send(PlayerAttackEvent::RightPunch);
    }
}

#[derive(Resource, Debug, Default)]
struct InputInterval {
    map: HashMap<KeyCode, Timer>,
}

impl InputInterval {
    fn tick(&mut self, time: Res<Time>) {
        let map: &mut HashMap<KeyCode, Timer> = &mut self.map;
        let key_codes = map.keys().copied().collect::<Vec<_>>();

        for key_code in key_codes {
            if let Some(timer) = map.get_mut(&key_code) {
                timer.tick(time.delta());
                if timer.finished() {
                    map.remove(&key_code);
                }
            };
        }
    }

    fn check(&mut self, key: KeyCode) -> bool {
        if self.map.contains_key(&key) {
            false
        } else {
            self.map
                .insert(key, Timer::from_seconds(0.5, TimerMode::Once));
            true
        }
    }
}
