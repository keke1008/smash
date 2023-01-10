mod assets;
mod constants;

use crate::state::AppState;

pub(in crate::game) use self::assets::UiAssets;
use self::constants::{FONT_COLOR, FONT_SIZE, PADDING};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Debug)]
pub(super) struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, setup)
            .add_exit_system(AppState::InGame, despawn)
            .add_system(update.run_in_state(AppState::InGame));
    }
}

#[derive(Component, Debug)]
struct UiRoot;

#[derive(Component, Default)]
struct UiTimer(Timer);

#[derive(Component, Debug, Default)]
struct Elapsed(u32);

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
                .insert(Elapsed(0))
                .insert(TextBundle::from_section(
                    "",
                    TextStyle {
                        font: assets.font.clone_weak(),
                        font_size: FONT_SIZE,
                        color: FONT_COLOR,
                    },
                ));
        });
}

fn update(mut query: Query<(&mut UiTimer, &mut Elapsed, &mut Text)>, time: Res<Time>) {
    let Ok((mut timer, mut elapsed, mut text)) = query.get_single_mut() else {
        return;
    };
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        elapsed.0 += 1;
        text.sections[0].value = elapsed.0.to_string();
    }
}

fn despawn(mut commands: Commands, query: Query<Entity, With<UiRoot>>) {
    let Ok(ui_entity) = query.get_single() else {
        return;
    };
    commands.entity(ui_entity).despawn_recursive();
}
