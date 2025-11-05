use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use pac_man::{
    BACKGROUND_COLOR, EatPelletEvent, FontAssets, GameState, LanguageSettings, Score,
    WINDOW_HEIGHT, WINDOW_WIDTH, cleanup_menu_ui, handle_eat_pellet_message, handle_menu_button,
    handle_player_input, load_font_assets, load_map_data, player_update, setup_map_ui,
    setup_menu_ui, sync_player_ui,
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
        .insert_resource(Score { value: 0 })
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
                handle_player_input,                            // 输入
                player_update.after(handle_player_input),       // 移动
                sync_player_ui.after(player_update),            // 同步位置
                handle_eat_pellet_message.after(player_update), // 处理吃豆事件
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
