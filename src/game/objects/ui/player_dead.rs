use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{game::state::PlayerState, state::AppState, utils::despawn_recursive};

use super::{
    assets::UiAssets,
    constants::{
        BUTTON_HOVERED, BUTTON_NORMAL, FONT_COLOR, FONT_SIZE_LARGE, FONT_SIZE_REGULAR, PADDING,
    },
    elapsed::Elapsed,
};

#[derive(Debug)]
pub(super) struct PlayerDeadUiPlugin;

impl Plugin for PlayerDeadUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(PlayerState::Dead, setup)
            .add_exit_system(PlayerState::Dead, despawn_recursive::<UiRoot>)
            .add_system(update.run_in_state(PlayerState::Dead));
    }
}

#[derive(Component, Debug)]
struct UiRoot;

#[derive(Component, Debug)]
struct RestartButton;

fn setup(mut commands: Commands, assets: Res<UiAssets>, elapsed: Res<Elapsed>) {
    commands
        .spawn((UiRoot, Name::new("UiRoot")))
        .insert(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        format!("SCORE:{}", elapsed.get()),
                        TextStyle {
                            font: assets.font.clone_weak(),
                            font_size: FONT_SIZE_LARGE,
                            color: FONT_COLOR,
                        },
                    ));
                });

            builder
                .spawn(RestartButton)
                .insert(ButtonBundle {
                    background_color: Color::RED.into(),
                    style: Style {
                        size: Size::new(Val::Percent(80.0), Val::Auto),
                        padding: UiRect::all(Val::Px(PADDING)),
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(
                        TextBundle::from_section(
                            "!RESTART!",
                            TextStyle {
                                font: assets.font.clone_weak(),
                                font_size: FONT_SIZE_REGULAR,
                                color: FONT_COLOR,
                            },
                        )
                        .with_text_alignment(TextAlignment::CENTER),
                    );
                });
        });
}

fn update(
    mut commands: Commands,
    mut button: Query<
        (&Interaction, &mut BackgroundColor),
        (With<RestartButton>, Changed<Interaction>),
    >,
) {
    let Ok((interaction, mut background_color)) = button.get_single_mut() else {
        return;
    };
    match interaction {
        Interaction::Clicked => commands.insert_resource(NextState(AppState::InGame)),
        Interaction::Hovered => *background_color = BUTTON_HOVERED.into(),
        Interaction::None => *background_color = BUTTON_NORMAL.into(),
    }
}
