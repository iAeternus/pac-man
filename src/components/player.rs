use bevy::ecs::component::Component;
use glam::IVec2;

use crate::{Movement, check_position, components::TileType};

/// 玩家
#[derive(Debug, Component)]
pub struct Player {
    /// 当前地图格坐标
    pub tile_pos: IVec2,
    pub movement: Movement,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            tile_pos: IVec2::new(x, y),
            movement: Movement::new(6.0),
        }
    }

    pub fn try_move(&mut self, tiles: &Vec<Vec<TileType>>) -> Option<IVec2> {
        let movement = &mut self.movement;
        if movement.direction == IVec2::ZERO || !movement.is_moving {
            return None;
        }

        let new_pos = self.tile_pos + movement.direction;
        let height = tiles.len();
        let width = tiles.get(0).map_or(0, |l| l.len());

        if !check_position(new_pos.x, new_pos.y, width, height)
            || tiles[new_pos.y as usize][new_pos.x as usize] == TileType::Wall
        {
            movement.stop_moving();
            return None;
        }

        Some(new_pos)
    }
}
