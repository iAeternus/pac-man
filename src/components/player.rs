use bevy::ecs::component::Component;
use glam::IVec2;

use crate::{check_position, components::TileType};

/// 玩家
#[derive(Debug, Component)]
pub struct Player {
    /// 当前地图格坐标
    pub tile_pos: IVec2,
    /// 方向
    pub direction: IVec2,
    /// 速度 (移动间隔的倒数，值越大移动越快)
    pub speed: f32,
    /// 是否正在移动
    pub is_moving: bool,
    /// 累积时间
    pub accumulated_time: f32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            tile_pos: IVec2::new(x, y),
            direction: IVec2::ZERO,
            speed: 6.0,
            is_moving: false,
            accumulated_time: 0.0,
        }
    }

    pub fn set_direction(&mut self, dir: IVec2) {
        self.direction = dir;
        self.is_moving = true;
    }

    pub fn stop_moving(&mut self) {
        self.is_moving = false;
    }

    pub fn try_move(&mut self, tiles: &Vec<Vec<TileType>>) -> Option<IVec2> {
        if self.direction == IVec2::ZERO || !self.is_moving {
            return None;
        }

        let new_pos = self.tile_pos + self.direction;
        let height = tiles.len();
        let width = tiles.get(0).map_or(0, |l| l.len());

        if !check_position(new_pos.x, new_pos.y, width, height)
            || tiles[new_pos.y as usize][new_pos.x as usize] == TileType::Wall
        {
            self.stop_moving();
            return None;
        }

        Some(new_pos)
    }

    /// 获取移动间隔（秒），速度越快间隔越短
    pub fn get_move_interval(&self) -> f32 {
        1.0 / self.speed
    }

    /// 是否可以移动（累积时间达到间隔）
    pub fn can_move(&self) -> bool {
        self.accumulated_time >= self.get_move_interval()
    }

    /// 重置累积时间
    pub fn reset_accumulated_time(&mut self) {
        self.accumulated_time = 0.0;
    }
}
