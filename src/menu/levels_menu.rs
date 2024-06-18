use bevy::prelude::*;

use crate::{data::get_levels_data, AppState};



use super::{interactions::interact_with_back_button, styles::*};

pub struct LevelsMenuPlugin;

#[allow(dead_code,path_statements)]
impl Plugin for LevelsMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        //
        .add_systems(OnEnter(AppState::Levels), spawn_levels_menu)
        .add_systems(Update,interact_with_back_button.run_if(in_state(AppState::Levels)))
        .add_systems(OnExit(AppState::Levels), despawn_levels_menu)
        //
        ;
    }
}

pub mod levels_menu_buttons {
    use bevy::prelude::Component;

    #[derive(Component)]
    pub struct BackButtonComponent;

}

#[derive(Component)]
pub struct LevelsMenuComponent;

pub fn spawn_levels_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _main_menu_entity = build_levels_menu(&mut commands, &asset_server);
}

pub fn despawn_levels_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<LevelsMenuComponent>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_levels_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let game_ui_entity = commands
        .spawn((
            NodeBundle {
                style: PAUSE_MENU_STYLE,
                ..default()
            },
            LevelsMenuComponent {},
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
                    levels_menu_buttons::BackButtonComponent,
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

        // end
        })
        .id();
    println!("{:?}",get_levels_data());
    game_ui_entity
}
