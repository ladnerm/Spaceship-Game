use bevy::prelude::*;

use crate::player::components::Player;
use crate::components::*;


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


    commands.spawn(TextBundle::from_section(
        "Score: ",
        TextStyle {
            font_size: 20.0,

            ..default()
        })
        .with_style( Style {
            position_type: PositionType::Absolute,
            margin: UiRect::new(
                Val::Px(5.0),
                Val::Px(5.0),
                Val::Px(5.0),
                Val::Px(5.0)
            ),
            ..default()
        })
    )
    .insert(PlayingComponents());

    commands.spawn(TextBundle::from_section(
        string_score,
        TextStyle {
            font_size: 20.0,

            ..default()
        })
        .with_style( Style {
            position_type: PositionType::Absolute,
            margin: UiRect::new(
                Val::Px(80.0),
                Val::Px(5.0),
                Val::Px(5.0),
                Val::Px(5.0)
            ),
            ..default()
        })
    )
    .insert(Texts());
}