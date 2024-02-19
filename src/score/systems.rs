use bevy::prelude::*;

use crate::components::*;
use super::resources::*;
use super::components::*;
use crate::player::components::Player;

pub fn display_score(
    mut commands: Commands,
    query_player: Query<&mut Player>,
    query_text: Query<Entity, With<Texts>>,
) {
    let mut score: i8 = 0;

    for player in &query_player {
        score = player.score;
    }

    for text_item in query_text.iter() {
        commands.entity(text_item).despawn();
    }

    let string_score = score.to_string();

    commands
        .spawn(
            TextBundle::from_section(
                "Score: ",
                TextStyle {
                    font_size: 20.0,

                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                margin: UiRect::new(Val::Px(5.0), Val::Px(5.0), Val::Px(5.0), Val::Px(5.0)),
                ..default()
            }),
        )
        .insert(PlayingComponents);

    commands
        .spawn(
            TextBundle::from_section(
                string_score,
                TextStyle {
                    font_size: 20.0,

                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                margin: UiRect::new(Val::Px(80.0), Val::Px(5.0), Val::Px(5.0), Val::Px(5.0)),
                ..default()
            }),
        )
        .insert(Texts);
}

pub fn update_high_score(player_query: Query<&Player>, mut high_score: ResMut<HighScore>) {
    for player in &player_query {
        if player.score > high_score.score {
            high_score.score = player.score;
        }
    }
}

pub fn display_high_score(
    mut commands: Commands,
    high_score: Res<HighScore>,
    high_score_query: Query<Entity, With<HighScoreText>>,
) {
    let high_score_string = high_score.score.to_string();

    for high_score_text in high_score_query.iter() {
        commands.entity(high_score_text).despawn();
    }

    commands
        .spawn(
            TextBundle::from_section(
                high_score_string,
                TextStyle {
                    font_size: 20.0,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                margin: UiRect::new(Val::Px(118.0), Val::Px(0.0), Val::Px(31.0), Val::Px(5.0)),
                ..default()
            }),
        )
        .insert(HighScoreText);

    commands.spawn(
        TextBundle::from_section(
            "High Score:",
            TextStyle {
                font_size: 20.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            margin: UiRect::new(Val::Px(5.0), Val::Px(0.0), Val::Px(30.0), Val::Px(5.0)),
            ..default()
        }),
    );
}
