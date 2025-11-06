use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use pac_man::{
    BACKGROUND_COLOR, EatPelletEvent, FontAssets, GameState, LanguageSettings, Score,
    WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH, cleanup_menu_ui, ghost_ai_system, ghost_move_system,
    handle_eat_pellet_message, handle_menu_button, handle_player_input, load_font_assets,
    load_map_data, player_update, setup_map_ui, setup_menu_ui, spawn_ghost_ui, sync_player_ui,
    update_ghost_ui,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: WINDOW_TITLE.into(),
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
                handle_player_input,
                player_update.after(handle_player_input),
                sync_player_ui,
                handle_eat_pellet_message.after(player_update),
            )
                .run_if(in_state(GameState::Playing)),
        )
        // 幽灵系统
        .add_systems(
            Update,
            (
                ghost_ai_system,
                ghost_move_system.after(ghost_ai_system),
                spawn_ghost_ui,
                update_ghost_ui.after(ghost_move_system),
            )
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    // 创建共享的2D相机
    commands.spawn(Camera2d);
}
