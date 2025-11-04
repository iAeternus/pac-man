use std::path::Path;

use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use pac_man::{
    FontAssets, GameState, LanguageSettings, MapDataResource, MapLoader, QuitButton, StartButton,
    TextMapLoader, cleanup_menu_ui, load_font_assets, setup_map_ui, setup_menu_ui,
};

fn main() -> anyhow::Result<()> {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pac-Man".into(),
                resolution: (960, 640).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .init_state::<GameState>()
        .init_resource::<LanguageSettings>()
        .init_resource::<FontAssets>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (load_font_assets, setup_camera, load_map_data))
        // 菜单系统
        .add_systems(OnEnter(GameState::Menu), setup_menu_ui)
        .add_systems(Update, handle_menu_button.run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), cleanup_menu_ui)
        // 地图系统
        .add_systems(OnEnter(GameState::Playing), setup_map_ui)
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
                *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));

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
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
            Interaction::None => {
                // 恢复默认颜色
                *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
        }
    }
}

fn load_map_data(mut commands: Commands) {
    const MAP_PATH: &'static str = "assets/map/pacman.map";

    let loader = TextMapLoader;
    let map_path = Path::new(MAP_PATH);
    let map_data = loader.load_map(map_path).expect("Failed to load map");

    commands.insert_resource(MapDataResource(map_data));
}
