use bevy::prelude::*;

use crate::{
    BACKGROUND_COLOR, BUTTON_TEXT_COLOR, QUIT_BUTTON_BORDER_COLOR, START_BUTTON_BORDER_COLOR, TITLE_COLOR, fonts::{FontAssets, get_font_for_language}, localization::LanguageSettings
};

#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct QuitButton;

/// 设置菜单UI
pub fn setup_menu_ui(
    mut commands: Commands,
    language_settings: Res<LanguageSettings>,
    font_assets: Res<FontAssets>,
) {
    let font = get_font_for_language(&language_settings, &font_assets);
    let texts = language_settings.get_texts();

    // 根UI容器
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BACKGROUND_COLOR),
            MenuUI,
        ))
        .with_children(|parent| {
            // 游戏标题
            parent.spawn((
                Text::new(texts.game_title),
                TextFont {
                    font: font.clone(),
                    font_size: 80.0,
                    ..default()
                },
                TextColor(TITLE_COLOR),
            ));

            // Start 按钮
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BorderColor::all(START_BUTTON_BORDER_COLOR),
                    StartButton,
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(texts.start_button),
                        TextFont {
                            font: font.clone(),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(BUTTON_TEXT_COLOR),
                    ));
                });

            // Quit 按钮
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BorderColor::all(QUIT_BUTTON_BORDER_COLOR),
                    QuitButton,
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("Quit"),
                        TextFont {
                            font: font.clone(),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(BUTTON_TEXT_COLOR),
                    ));
                });
        });
}

pub fn cleanup_menu_ui(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
