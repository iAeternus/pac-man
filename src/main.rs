use bevy::prelude::*;
use pac_man::{FontAssets, GameState, LanguageSettings, load_font_assets, setup_menu_ui};

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
        .init_state::<GameState>()
        .init_resource::<LanguageSettings>()
        .init_resource::<FontAssets>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (load_font_assets, setup_camera))
        .add_systems(OnEnter(GameState::Menu), setup_menu_ui)
        .run();

    Ok(())
}

fn setup_camera(mut commands: Commands) {
    // 创建共享的2D相机
    commands.spawn(Camera2d);
}
