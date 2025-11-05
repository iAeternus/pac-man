use bevy::prelude::*;

use crate::{GameState, HOVERED_COLOR, NONE_COLOR, PRESSED_COLOR, QuitButton, StartButton};

/// 处理菜单页面的按钮
pub fn handle_menu_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&StartButton>,
            Option<&QuitButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, mut color, start_btn, quit_btn) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // 按下时的视觉反馈
                *color = BackgroundColor(PRESSED_COLOR);

                if start_btn.is_some() {
                    info!("Start pressed -> Switching to Playing");
                    next_state.set(GameState::Playing);
                }

                if quit_btn.is_some() {
                    info!("Quit pressed -> Exiting game");
                    exit.write(AppExit::Success);
                }
            }
            Interaction::Hovered => {
                // 悬停效果
                *color = BackgroundColor(HOVERED_COLOR);
            }
            Interaction::None => {
                // 恢复默认颜色
                *color = BackgroundColor(NONE_COLOR);
            }
        }
    }
}
