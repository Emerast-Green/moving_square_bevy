use bevy::prelude::*;

use crate::{
    menu::{
        interactions::{interact_with_main_menu_button, interact_with_resume_button, menu_buttons},
        styles::{
            get_normal_text_style, NORMAL_BUTTON_COLOR, NORMAL_BUTTON_STYLE, PAUSE_MENU_STYLE,
        },
    },
    AppState, HudState, SimulationState,
};

#[derive(Component)]
pub struct PauseMenuComponent;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        // .
        .add_systems(OnEnter(HudState::Pause), spawn_pause_menu)
        .add_systems(Update, (interact_with_main_menu_button,interact_with_resume_button,input_resume_game)
            .run_if(in_state(AppState::Game))
            .run_if(in_state(HudState::Pause)))
        .add_systems(Update, input_pause_game
            .run_if(in_state(SimulationState::Running))
            .run_if(in_state(AppState::Game))
            .run_if(in_state(HudState::None)))
        .add_systems(OnExit(HudState::Pause), despawn_pause_menu)
        .add_systems(OnExit(AppState::Game), despawn_pause_menu)
        // .
        ;
    }
}

pub fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut pause_state: ResMut<NextState<SimulationState>>,
) {
    let _main_menu_entity = build_pause_menu(&mut commands, &asset_server);
    pause_state.set(SimulationState::Paused)
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<PauseMenuComponent>>,
    mut pause_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
    pause_state.set(SimulationState::Running)
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
                    menu_buttons::ResumeButtonComponent,
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
                    menu_buttons::MainMenuButtonComponent,
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
    mut hud_state_next_state: ResMut<NextState<HudState>>,
    hud_state: Res<State<HudState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) && *hud_state.get() == HudState::None {
        hud_state_next_state.set(HudState::Pause)
    }
}

pub fn input_resume_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut hud_state_next_state: ResMut<NextState<HudState>>,
    hud_state: Res<State<HudState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) && *hud_state.get() == HudState::Pause {
        hud_state_next_state.set(HudState::None)
    }
}
