mod game;
mod state;
mod utils;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;

use self::state::AppState;

pub fn run_app() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                canvas: Some("#canvas".into()),
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RunAwayPlugin)
        .run();
}

#[derive(Debug, Default)]
struct RunAwayPlugin;

impl Plugin for RunAwayPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(AppState::Loading)
            .add_plugin(game::GamePlugin);
    }
}
