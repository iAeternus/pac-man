use bevy::ecs::component::Component;
use glam::IVec2;

use crate::{MapData, Movement, TryMove};

/// 玩家
#[derive(Debug, Component)]
pub struct Player {
    /// 当前地图格坐标
    pub tile_pos: IVec2,
    /// 移动组件
    pub movement: Movement,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            tile_pos: IVec2::new(x, y),
            movement: Movement::new(6.0),
        }
    }
}

impl TryMove for Player {
    fn try_move(&mut self, map_data: &MapData) -> Option<IVec2> {
        if self.movement.direction == IVec2::ZERO || !self.movement.is_moving {
            return None;
        }

        let new_pos = self.tile_pos + self.movement.direction;
        if !map_data.is_valid_position(new_pos.x, new_pos.y)
            || map_data.is_wall(new_pos.x as usize, new_pos.y as usize)
        {
            self.movement.stop_moving();
            return None;
        }

        Some(new_pos)
    }
}
