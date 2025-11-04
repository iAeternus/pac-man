use bevy::state::state::States;

/// 游戏状态
#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    #[default]
    Menu, // 主菜单
    Playing,  // 游戏中
    Paused,   // 暂停
    GameOver, // 游戏结束
}