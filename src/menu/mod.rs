mod levels_menu;
mod main_menu;
mod score_menu;
pub(crate) mod styles;
pub(crate) mod interactions;

use bevy::prelude::*;
use levels_menu::LevelsMenuPlugin;
use main_menu::MainMenuPlugin;
use score_menu::ScoreMenuPlugin;
pub(crate) use score_menu::SpawnScoreMenuEvent;

pub struct MenuPLugin;

impl Plugin for MenuPLugin {
    fn build(&self, app: &mut App) {
        app
        //
        .add_plugins((MainMenuPlugin,LevelsMenuPlugin,ScoreMenuPlugin))
        //
        ;
    }
}