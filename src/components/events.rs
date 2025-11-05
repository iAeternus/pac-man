use bevy::ecs::message::Message;
use glam::IVec2;

use crate::{PelletType, is_power_pellet};

/// 吃豆子事件
#[derive(Message)]
pub struct EatPelletEvent {
    /// 当前被吃掉豆子的坐标
    pub position: IVec2,
    /// 豆子类型
    pub pellet_type: PelletType,
}

impl EatPelletEvent {
    pub fn new(pos: IVec2) -> Self {
        Self {
            position: pos,
            pellet_type: if is_power_pellet(pos.x as usize, pos.y as usize) {
                PelletType::Power
            } else {
                PelletType::General
            },
        }
    }
}
