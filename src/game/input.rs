mod events;
mod gesture;
mod keyboard;

use bevy::prelude::*;

pub(in crate::game) use self::events::{
    PlayerAttackEvent, PlayerMovementEvent, PlayerRotationEvent,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerAttackEvent>()
            .add_event::<PlayerMovementEvent>()
            .add_event::<PlayerRotationEvent>();
        app.add_plugin(keyboard::KeyboardInputPlugin)
            .add_plugin(gesture::GestureInputPlugin);
    }
}
