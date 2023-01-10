use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::game::state::PlayerState;

use super::{
    assets::UiAssets,
    constants::{FONT_COLOR, FONT_SIZE_REGULAR, PADDING},
    elapsed::Elapsed,
};

#[derive(Debug)]
pub(super) struct PlayerLivingUiPlugin;

impl Plugin for PlayerLivingUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(PlayerState::Living, setup)
            .add_exit_system(PlayerState::Living, despawn)
            .add_system(update.run_in_state(PlayerState::Living));
    }
}

#[derive(Component, Debug)]
struct UiRoot;

#[derive(Component, Debug)]
struct UiTimer(Timer);

fn setup(mut commands: Commands, assets: Res<UiAssets>) {
    commands
        .spawn((UiRoot, Name::new("UiRoot")))
        .insert(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                padding: UiRect::all(Val::Px(PADDING)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn(UiTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
                .insert(TextBundle::from_section(
                    "",
                    TextStyle {
                        font: assets.font.clone_weak(),
                        font_size: FONT_SIZE_REGULAR,
                        color: FONT_COLOR,
                    },
                ));
        });
}

fn update(
    mut elapsed: ResMut<Elapsed>,
    mut timer: Query<(&mut UiTimer, &mut Text)>,
    time: Res<Time>,
) {
    let Ok((mut timer, mut text)) = timer.get_single_mut() else {
        return;
    };
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        elapsed.increment();
        text.sections[0].value = elapsed.get().to_string();
    }
}

fn despawn(mut commands: Commands, query: Query<Entity, With<UiRoot>>) {
    let Ok(ui_entity) = query.get_single() else {
        return;
    };
    commands.entity(ui_entity).despawn_recursive();
}
