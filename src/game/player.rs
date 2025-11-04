use bevy::ecs::component::Component;
use glam::{IVec2, Vec2};

use crate::game::TileType;

/// 玩家
#[derive(Debug)]
pub struct Player {
    /// 当前地图格坐标
    pub tile_pos: IVec2,
    /// 速度
    pub speed: f32,
    /// 方向
    pub direction: IVec2,
}

impl Player {
    pub fn new(y: i32, x: i32) -> Self {
        Self {
            tile_pos: IVec2::new(y, x),
            speed: 4.0,
            direction: IVec2::ZERO,
        }
    }

    pub fn set_direction(&mut self, dir: IVec2) {
        self.direction = dir;
    }

    pub fn try_move(&mut self, tiles: Vec<Vec<TileType>>) {
        if self.direction == IVec2::ZERO {
            return;
        }

        let new_pos = self.tile_pos + self.direction;
        let height = tiles.len();
        let width = tiles.get(0).map_or(0, |l| l.len());

        // 边界检查
        if new_pos.y < 0
            || new_pos.y as usize >= height
            || new_pos.x < 0
            || new_pos.x as usize >= width
        {
            return;
        }

        // 墙体检测
        if tiles[new_pos.y as usize][new_pos.x as usize] == TileType::Wall {
            return;
        }

        self.tile_pos = new_pos;
    }
}

#[derive(Component)]
pub struct PlayerComponent {
    pub player: Player,
}
