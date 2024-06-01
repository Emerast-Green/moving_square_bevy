use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};
use data::LoaderPlugin;
use game::{GamePlugin, ObstaclePlugin, PlayerPlugin};

mod data;
mod game;
mod menu;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Moving Square [Bevy] beta".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        // States
        .init_state::<AppState>()
        .init_state::<SimulationState>()
        // FixedUpdate Time
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(Startup, spawn_camera)
        // Plugins
        .add_plugins((GamePlugin,LoaderPlugin))
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppState {
    MainMenu,
    Levels,
    #[default]
    Game,
}

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
