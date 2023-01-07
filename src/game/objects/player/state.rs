use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use iyes_loopless::prelude::*;

use crate::state::AppState;

#[derive(Debug)]
pub(super) struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tick.run_in_state(AppState::InGame));
    }
}

fn tick(mut state: Query<&mut CurrentPlayerState, Changed<CurrentPlayerState>>, time: Res<Time>) {
    let Ok(mut current) = state.get_single_mut() else {
        return;
    };
    current.tick(time.delta());
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum PlayerState {
    #[default]
    Stay,
    Run,
    LeftPunch,
    RightPunch,
}

impl PlayerState {
    fn is_finite(&self) -> bool {
        use PlayerState::*;
        matches!(self, LeftPunch | RightPunch)
    }

    fn duration(&self) -> f32 {
        use PlayerState::*;
        match self {
            Stay | Run => f32::INFINITY,
            LeftPunch | RightPunch => 10.0 / 24.0,
        }
    }
}

#[derive(Component, Debug, Default)]
pub(super) struct CurrentPlayerState {
    finite: Option<(PlayerState, Stopwatch)>,
    infinite: PlayerState,
}

impl CurrentPlayerState {
    pub(super) fn get(&self) -> PlayerState {
        self.finite
            .as_ref()
            .map_or(self.infinite, |(state, _)| *state)
    }

    pub(super) fn set(&mut self, state: PlayerState) {
        if state.is_finite() {
            self.finite = Some((state, Stopwatch::new()));
        } else {
            self.infinite = state;
        }
    }

    fn tick(&mut self, delta: Duration) {
        if let Some((finite_state, elapsed)) = &mut self.finite {
            elapsed.tick(delta);
            if elapsed.elapsed_secs() >= finite_state.duration() {
                self.finite = None;
            }
        };
    }
}
