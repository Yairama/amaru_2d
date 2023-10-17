use crate::resources::scoreboard::Scoreboard;
use bevy::prelude::{Query, Res, Text};

pub(crate) fn update_scoreboard(score: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.score.to_string();
}
