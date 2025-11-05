use bevy::ecs::component::Component;
use glam::IVec2;

use crate::POWER_PELLET_POSITIONS;

/// 豆子类型
pub enum PelletType {
    General,
    Power,
}

/// 豆子
#[derive(Component)]
pub struct Pellet {
    pub position: IVec2,
    pub pellet_type: PelletType,
}

impl Pellet {
    pub fn new(position: IVec2, pellet_type: PelletType) -> Self {
        Self {
            position,
            pellet_type,
        }
    }
}

/// 判断是否为能量豆
pub fn is_power_pellet(x: usize, y: usize) -> bool {
    POWER_PELLET_POSITIONS.contains(&(x, y))
}
