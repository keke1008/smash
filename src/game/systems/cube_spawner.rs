mod constants;
mod spawn_control;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{game::state::PlayerState, state::AppState};

use self::{
    constants::{MetalCube, SpawnableCubes, StoneCube, WoodenCube},
    spawn_control::SpawnControl,
};

#[derive(Debug)]
pub(super) struct CubeSpawnerPlugin;

impl Plugin for CubeSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, setup)
            .add_exit_system(AppState::InGame, despawn)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .run_in_state(PlayerState::Living)
                    .with_system(update::<WoodenCube>)
                    .with_system(update::<StoneCube>)
                    .with_system(update::<MetalCube>)
                    .into(),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(SpawnControl::<WoodenCube>::default());
    commands.insert_resource(SpawnControl::<StoneCube>::default());
    commands.insert_resource(SpawnControl::<MetalCube>::default());
}

fn despawn(mut commands: Commands) {
    commands.remove_resource::<SpawnControl<WoodenCube>>();
    commands.remove_resource::<SpawnControl<StoneCube>>();
    commands.remove_resource::<SpawnControl<MetalCube>>();
}

fn update<Cube: SpawnableCubes + 'static>(
    mut commands: Commands,
    mut control: ResMut<SpawnControl<Cube>>,
    time: Res<Time>,
) {
    control.tick(time.delta());
    if let Some(amount) = control.get_spawn_amount() {
        let spawn_index = spawn_control::generate_spawn_index::<Cube>(amount);
        spawn_control::generate_spawn_commands::<Cube>(&spawn_index).for_each(|command| {
            commands.add(command);
        });
    }
}
