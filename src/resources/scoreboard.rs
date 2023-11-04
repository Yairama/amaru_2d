use bevy::prelude::{Color, Resource, Val};

//scoreboard
pub(crate) const SCOREBOARD_FONT_SIZE: f32 = 40.0;
pub(crate) const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
pub(crate) const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
pub(crate) const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
#[derive(Resource)]
pub struct Scoreboard {
    pub score: usize,
}
