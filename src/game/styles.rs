use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.35, 0.35);

pub const NORMAL_BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.align_content = AlignContent:: Center;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.justify_items = JustifyItems::Center;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style
};

pub const NORMAL_IMAGE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(64.0);
    style.height = Val::Px(64.0);
    style.margin = UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0));
    style
};

pub fn get_normal_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::WHITE,
    }
}

pub const TITLE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.align_content = AlignContent:: Center;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.justify_items = JustifyItems::Center;
    style.width = Val::Px(600.0);
    style.height = Val::Px(120.0);
    style
};

pub const MAIN_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};

pub const PAUSE_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(40.0);
    style.height = Val::Percent(60.0);
    style.align_self = AlignSelf::Center;
    style.justify_self = JustifySelf::Center;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};

pub const SCORE_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(40.0);
    style.height = Val::Percent(60.0);
    style.align_self = AlignSelf::Center;
    style.justify_self = JustifySelf::Center;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};

pub const GAME_UI_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.align_self = AlignSelf::Start;
    style.justify_self = JustifySelf::Start;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceBetween;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};

pub const COUNTER_STARS_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(40.0);
    style.height = Val::Percent(25.0);
    style.align_self = AlignSelf::Start;
    style.justify_self = JustifySelf::Start;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Start;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style.border = UiRect {
        bottom: Val::Px(5.0),
        top: Val::Px(5.0),
        left: Val::Px(5.0),
        right: Val::Px(5.0)
    };
    style
};

pub const COUNTER_ENEMIES_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(40.0);
    style.height = Val::Percent(25.0);
    style.align_self = AlignSelf::Start;
    style.justify_self = JustifySelf::Start;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::End;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style.border = UiRect {
        bottom: Val::Px(5.0),
        top: Val::Px(5.0),
        left: Val::Px(5.0),
        right: Val::Px(5.0)
    };
    style
};