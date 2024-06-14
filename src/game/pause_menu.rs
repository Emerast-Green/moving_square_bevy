use bevy::prelude::*;

use crate::{
    menu::{
        interactions::{interact_with_main_menu_button, interact_with_resume_button},
        styles::{
            get_normal_text_style, NORMAL_BUTTON_COLOR, NORMAL_BUTTON_STYLE, PAUSE_MENU_STYLE,
        },
    },
    AppState, SimulationState,
};

pub mod pause_menu_buttons {
    use bevy::prelude::Component;

    #[derive(Component)]
    pub struct ResumeButtonComponent;

    #[derive(Component)]
    pub struct MainMenuButtonComponent;
}

#[derive(Component)]
pub struct PauseMenuComponent;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        // .
        .add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu)
        .add_systems(Update, (interact_with_main_menu_button,interact_with_resume_button,input_resume_game)
            .run_if(in_state(SimulationState::Paused))
            .run_if(in_state(AppState::Game)))
        .add_systems(Update, input_pause_game
            .run_if(in_state(SimulationState::Running))
            .run_if(in_state(AppState::Game)))
        .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu)
        .add_systems(OnExit(AppState::Game), despawn_pause_menu)
        // .
        ;
    }
}

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _main_menu_entity = build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<PauseMenuComponent>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let game_ui_entity = commands
        .spawn((
            NodeBundle {
                style: PAUSE_MENU_STYLE,
                ..default()
            },
            PauseMenuComponent {},
        ))
        .with_children(|parent| {
            // Play Button
            parent
                .spawn((
                    ButtonBundle {
                        style: NORMAL_BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    pause_menu_buttons::ResumeButtonComponent,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Resume",
                                get_normal_text_style(&asset_server),
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // Exit Button
            parent
                .spawn((
                    ButtonBundle {
                        style: NORMAL_BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    pause_menu_buttons::MainMenuButtonComponent,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Exit",
                                get_normal_text_style(&asset_server),
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();
    game_ui_entity
}

pub fn input_pause_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        simulation_state_next_state.set(SimulationState::Paused)
    }
}

pub fn input_resume_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        simulation_state_next_state.set(SimulationState::Running)
    }
}
