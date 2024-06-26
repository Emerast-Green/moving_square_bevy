
use bevy::{prelude::*, time::Stopwatch};

use crate::{
    menu::styles::{get_title_text_style, COUNTER_STARS_STYLE, GAME_UI_STYLE}, AppState
};

use super::coin::Score;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_systems(OnEnter(AppState::Game), spawn_player_hud)
            .add_systems(Update, (update_score_label,update_time).run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), despawn_player_hud)
            //.
            ;
    }
}

#[derive(Component)]
pub struct PlayerHud;

// #[derive(Component)]
// pub struct PlayerHudJumped;

#[derive(Component)]
pub struct HudPlayerScore;

#[derive(Component)]
pub struct HubPlayerTime {
    time: Stopwatch
}


pub fn spawn_player_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _main_menu_entity = build_player_hud(&mut commands, &asset_server);
}

pub fn despawn_player_hud(mut commands: Commands, main_menu_query: Query<Entity, With<PlayerHud>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_player_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let game_ui_entity = commands
        .spawn((
            NodeBundle {
                style: GAME_UI_STYLE,
                ..default()
            },
            PlayerHud {},
        ))
        .with_children(|parent| {
            // Stars Counter
            parent
                .spawn(NodeBundle {
                    style: COUNTER_STARS_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Score
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "0",
                                    get_title_text_style(&asset_server),
                                )],
                                justify: JustifyText::Center,
                                linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                            },
                            ..default()
                        },
                        HudPlayerScore {},
                    ));
                    // Time
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "0",
                                    get_title_text_style(&asset_server),
                                )],
                                justify: JustifyText::Center,
                                linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                            },
                            ..default()
                        },
                        HubPlayerTime {
                            time: Stopwatch::new()
                        },
                    ));
                });
        })
        .id();
    game_ui_entity
}

// pub fn update_jumped_label(
//     mut jumped_ui_query: Query<&mut Text, With<PlayerHudJumped>>,
//     jumped_query: Query<&JumpLock, With<PlayerComponent>>,
// ) {
//     if let Ok(mut jumped_text) = jumped_ui_query.get_single_mut() {
//         jumped_text.sections[0].value = jumped_query.get_single().unwrap().0.to_string();
//     }
// }

pub fn update_score_label(
    mut score_ui_query: Query<&mut Text, With<HudPlayerScore>>,
    score_resource: Res<Score>,
) {
    if !score_resource.is_changed(){return;};
    if let Ok(mut jumped_text) = score_ui_query.get_single_mut() {
        jumped_text.sections[0].value = score_resource.current.to_string();
    }
}

pub fn update_time(
    mut time_ui_query: Query<(&mut Text, &mut HubPlayerTime), With<HubPlayerTime>>,
    game_time: Res<Time>
) {
    if let Ok((mut text, mut time)) = time_ui_query.get_single_mut() {
        time.time.tick(game_time.delta());
        text.sections[0].value = ((time.time.elapsed_secs()*100.0).floor()/100.0).to_string();
    }
}