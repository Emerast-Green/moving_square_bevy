mod levels_menu;
mod main_menu;
pub(crate) mod styles;
mod interactions;

use bevy::prelude::*;
use levels_menu::LevelsMenuPlugin;
use main_menu::MainMenuPlugin;

pub struct MenuPLugin;

impl Plugin for MenuPLugin {
    fn build(&self, app: &mut App) {
        app
        //
        .add_plugins((MainMenuPlugin,LevelsMenuPlugin))
        //
        ;
    }
}