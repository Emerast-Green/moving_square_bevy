use bevy::prelude::*;

use crate::{
    menu::styles::{get_title_text_style, COUNTER_STARS_STYLE, GAME_UI_STYLE},
    AppState,
};

use super::{JumpLock, PlayerComponent};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_systems(OnEnter(AppState::Game), spawn_player_hud)
            .add_systems(Update, update_jumped_label.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), despawn_player_hud)
            //.
            ;
    }
}

#[derive(Component)]
pub struct PlayerHud;

#[derive(Component)]
pub struct PlayerHudJumped;

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
                    // Text
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
                        PlayerHudJumped {},
                    ));
                });
        })
        .id();
    game_ui_entity
}

pub fn update_jumped_label(
    mut jumped_ui_query: Query<&mut Text, With<PlayerHudJumped>>,
    jumped_query: Query<&JumpLock, With<PlayerComponent>>,
) {
    if let Ok(mut jumped_text) = jumped_ui_query.get_single_mut() {
        jumped_text.sections[0].value = jumped_query.get_single().unwrap().0.to_string();
    }
}
