use bevy::app::Plugin;
use coin::CoinPLugin;
use door::DoorPlugin;
use obstacle::ObstaclePlugin;

pub mod coin;
pub mod door;
pub mod obstacle;

pub struct GameObjectsPlugin;

impl Plugin for GameObjectsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        //.
        .add_plugins(
            (
                CoinPLugin,
                ObstaclePlugin,
                DoorPlugin
            )
        )
        //.
        ;
    }
}
