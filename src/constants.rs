//! 全局常量定义

use bevy::color::Color;

// 窗口参数
pub const WINDOW_TITLE: &'static str = "Pac-Man";
pub const WINDOW_WIDTH: u32 = 960;
pub const WINDOW_HEIGHT: u32 = 640;
pub const BACKGROUND_COLOR: Color = Color::BLACK;
pub const TITLE_COLOR: Color = Color::srgb(1.0, 1.0, 0.0); // 黄色

// 按钮参数
pub const BUTTON_TEXT_COLOR: Color = Color::WHITE;
pub const START_BUTTON_BORDER_COLOR: Color = Color::srgb(0.2, 1.0, 0.2); // 亮绿色边框
pub const QUIT_BUTTON_BORDER_COLOR: Color = Color::srgb(1.0, 0.3, 0.3); // 红色边框
pub const PRESSED_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
pub const HOVERED_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
pub const NONE_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);

// 地图参数
pub const MAP_PATH: &'static str = "assets/map/pacman.map";
pub const TILE_SIZE: f32 = 20.0;
pub const HALF: f32 = TILE_SIZE / 2.0;
pub const WALL_THICKNESS: f32 = 1.0;
pub const WALL_COLOR: Color = Color::srgb(0.0, 0.6, 1.0);

// 道具参数
pub const PELLET_RADIUS: f32 = 2.0;
pub const POWER_PELLET_RADIUS: f32 = 4.0;
pub const PELLET_COLOR: Color = Color::WHITE;
pub const POWER_PELLET_COLOR: Color = Color::WHITE;
pub const POWER_PELLET_POSITIONS: [(usize, usize); 4] = [
    (1, 3),   // 左上角
    (26, 3),  // 右上角
    (1, 23),  // 左下角
    (26, 23), // 右下角
];

// 玩家参数
pub const PLAYER_RADIUS: f32 = 6.0;
pub const PLAYER_COLOR: Color = TITLE_COLOR;

// 渲染Z轴
pub const Z_MAP: f32 = 0.0;
pub const Z_WALL: f32 = 1.0;
pub const Z_PELLET: f32 = 2.0;
pub const Z_PLAYER: f32 = 3.0;

// 幽灵参数
pub const GHOST_RADIUS: f32 = 8.0;
pub const GHOST_EYE_RADIUS: f32 = 2.5;
pub const GHOST_PUPIL_RADIUS: f32 = 1.2;
pub const GHOST_LEFT_EYE_POSITION: (f32, f32) = (-2.5, 2.0);
pub const GHOST_RIGHT_EYE_POSITION: (f32, f32) = (2.5, 2.0);
pub const BLINKY_COLOR: Color = Color::srgb(0.890, 0.047, 0.047);
pub const PINKY_COLOR: Color = Color::srgb(0.988, 0.518, 0.831);
pub const INKY_COLOR: Color = Color::srgb(0.094, 0.816, 0.941);
pub const CLYDE_COLOR: Color = Color::srgb(0.988, 0.439, 0.0);
pub const GHOST_FRIGHTENED_COLOR: Color = Color::srgb(0.0, 0.0, 1.0);
pub const GHOST_FRIGHTENED_FLASH_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
pub const GHOST_EYE_WHITE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
pub const GHOST_EYE_PUPIL_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);
pub const GHOST_STROKE_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);
pub const Z_GHOST_BODY: f32 = 3.0;
pub const Z_GHOST_EYE_WHITE: f32 = 3.1;
pub const Z_GHOST_EYE_PUPIL: f32 = 3.2;
pub const GHOST_FRIGHTENED_FLASH_INTERVAL: f32 = 0.1;
