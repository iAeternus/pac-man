use std::path::Path;

use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use pac_man::{
    BACKGROUND_COLOR, EatPelletEvent, FontAssets, GameState, HOVERED_COLOR, LanguageSettings,
    MAP_PATH, MapLoader, NONE_COLOR, PRESSED_COLOR, QuitButton, Score, StartButton, TextMapLoader,
    WINDOW_HEIGHT, WINDOW_WIDTH, cleanup_menu_ui, handle_player_input, load_font_assets,
    player_update, setup_map_ui, setup_menu_ui, spawn_new_pellet, sync_player_ui,
};

fn main() -> anyhow::Result<()> {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pac-Man".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .init_state::<GameState>()
        .init_resource::<LanguageSettings>()
        .init_resource::<FontAssets>()
        .init_resource::<Score>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_message::<EatPelletEvent>()
        .add_systems(Startup, (load_font_assets, setup_camera, load_map_data))
        // 菜单系统
        .add_systems(OnEnter(GameState::Menu), setup_menu_ui)
        .add_systems(Update, handle_menu_button.run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), cleanup_menu_ui)
        // 地图系统
        .add_systems(OnEnter(GameState::Playing), setup_map_ui)
        // 玩家系统
        .add_systems(
            Update,
            (
                handle_player_input, // 输入
                player_update,       // 移动
                sync_player_ui,      // 同步位置
                spawn_new_pellet,    // 豆子更新
            )
                .run_if(in_state(GameState::Playing)),
        )
        .run();

    Ok(())
}

fn setup_camera(mut commands: Commands) {
    // 创建共享的2D相机
    commands.spawn(Camera2d);
}

/// 处理菜单页面的按钮
fn handle_menu_button(
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

fn load_map_data(mut commands: Commands) {
    let loader = TextMapLoader;
    let map_path = Path::new(MAP_PATH);
    let map_data = loader.load_map(map_path).expect("Failed to load map");

    commands.insert_resource(map_data);
}
