use bevy::ecs::component::Component;
use glam::IVec2;

use crate::game::TileType;

/// 玩家
#[derive(Debug, Component)]
pub struct Player {
    /// 当前地图格坐标
    pub tile_pos: IVec2,
    /// 方向
    pub direction: IVec2,
    /// 速度
    pub speed: f32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            tile_pos: IVec2::new(x, y),
            direction: IVec2::ZERO,
            speed: 4.0,
        }
    }

    pub fn set_direction(&mut self, dir: IVec2) {
        self.direction = dir;
    }

    pub fn try_move(&mut self, tiles: &Vec<Vec<TileType>>) -> Option<IVec2> {
        if self.direction == IVec2::ZERO {
            return None;
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
            return None;
        }

        // 墙体检测
        if tiles[new_pos.y as usize][new_pos.x as usize] == TileType::Wall {
            return None;
        }

        Some(new_pos)
    }
}
