use bevy::prelude::*;

use crate::{
    menu::styles::{
        get_normal_text_style, NORMAL_BUTTON_COLOR, NORMAL_BUTTON_STYLE, PAUSE_MENU_STYLE,
    },
    AppState, HudState, SimulationState,
};

use super::{
    interactions::{interact_with_main_menu_button, interact_with_replay_button, menu_buttons},
    styles::get_title_text_style,
};

pub struct ScoreMenuPlugin;

impl Plugin for ScoreMenuPlugin {
    fn build(&self, app: &mut App) {
        app
    // .
        .add_event::<SpawnScoreMenuEvent>()
        .add_systems(Update, handle_spawn_score_menu_event
            .run_if(in_state(AppState::Game))
        )
        .add_systems(Update, (interact_with_replay_button,interact_with_main_menu_button)
            .run_if(in_state(AppState::Game))
            .run_if(in_state(HudState::Score))
        )
        .add_systems(OnExit(HudState::Score), despawn_score_menu)
    // .
    ;
    }
}

#[derive(Event)]
pub struct SpawnScoreMenuEvent {
    pub points: usize,
    pub time: f32,
}

#[derive(Component)]
pub struct ScoreMenuComponent;

// handled by handle_spawn_score_menu_event
// pub fn spawn_score_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let _main_menu_entity = build_score_menu(&mut commands, &asset_server);
// }

pub fn despawn_score_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<ScoreMenuComponent>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_score_menu(
    _points: usize,
    time: f32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let game_ui_entity = commands
        .spawn((
            NodeBundle {
                style: PAUSE_MENU_STYLE,
                ..default()
            },
            ScoreMenuComponent {},
        ))
        .with_children(|parent| {
            // Time
            parent.spawn((TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        ((time * 100.0).floor() / 100.0).to_string()+"s",
                        get_title_text_style(&asset_server),
                    )],
                    justify: JustifyText::Center,
                    linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                },
                ..default()
            },));
            // Resume Button
            parent
                .spawn((
                    ButtonBundle {
                        style: NORMAL_BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    menu_buttons::ReplayButtonComponent,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Replay",
                                get_normal_text_style(&asset_server),
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // Replay Button
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

            // end
        })
        .id();
    game_ui_entity
}

pub fn handle_spawn_score_menu_event(
    mut event_reader: EventReader<SpawnScoreMenuEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,
    mut hud_next_state: ResMut<NextState<HudState>>,
) {
    match event_reader.read().last() {
        None => {}
        Some(event) => {
            let _main_menu_entity =
                build_score_menu(event.points, event.time, &mut commands, &asset_server);
            simulation_next_state.set(SimulationState::Paused);
            hud_next_state.set(HudState::Score)
        }
    };
}
