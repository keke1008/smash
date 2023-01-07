use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{game::tags::Player, state::AppState};

use super::{
    state::{CurrentPlayerState, PlayerState},
    PlayerAssets,
};

#[derive(Debug)]
pub(super) struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_animation.run_in_state(AppState::InGame));
    }
}

fn is_require_repeat(state: &PlayerState) -> bool {
    use PlayerState::*;
    match state {
        Stay | Run => true,
        LeftPunch | RightPunch => false,
    }
}

fn get_animation_cilp(state: &PlayerState, assets: &PlayerAssets) -> Handle<AnimationClip> {
    use PlayerState::*;
    let handle = match state {
        Stay => &assets.stay,
        Run => &assets.run,
        LeftPunch => &assets.left_punch,
        RightPunch => &assets.right_punch,
    };
    handle.clone_weak()
}

fn apply_animation(
    mut previous_state: Local<PlayerState>,
    player: Query<&CurrentPlayerState, With<Player>>,
    mut animation_player: Query<&mut AnimationPlayer>,
    assets: Res<PlayerAssets>,
) {
    let Ok(current_state) = player.get_single() else {
        return;
    };
    let current_state = current_state.get();
    if *previous_state == current_state {
        return;
    }
    *previous_state = current_state;

    let Ok(mut animation_player) = animation_player.get_single_mut() else {
        return;
    };

    let animation = get_animation_cilp(&current_state, &assets);
    animation_player.start(animation);
    if is_require_repeat(&current_state) {
        animation_player.repeat();
    }
}
