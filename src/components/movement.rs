use bevy::ecs::component::Component;
use glam::IVec2;

/// 移动组件
#[derive(Debug, Component, Clone)]
pub struct Movement {
    /// 方向
    pub direction: IVec2,
    /// 速度 (移动间隔的倒数，值越大移动越快)
    pub speed: f32,
    /// 是否正在移动
    pub is_moving: bool,
    /// 累积时间
    pub accumulated_time: f32,
}

impl Movement {
    pub fn new(speed: f32) -> Self {
        Self {
            direction: IVec2::ZERO,
            speed,
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
        self.accumulated_time = 0.0;
    }

    /// 获取移动间隔（秒），速度越快间隔越短
    pub fn get_move_interval(&self) -> f32 {
        1.0 / self.speed
    }

    /// 是否可以移动（累积时间达到间隔）
    pub fn can_move(&self) -> bool {
        self.accumulated_time >= self.get_move_interval()
    }

    /// 更新累积时间并返回是否可以移动
    pub fn update(&mut self, delta_secs: f32) -> bool {
        if !self.is_moving {
            self.accumulated_time = 0.0;
            return false;
        }

        self.accumulated_time += delta_secs;
        
        if self.can_move() {
            // 扣除移动间隔，返回true表示可以移动
            self.accumulated_time -= self.get_move_interval();
            true
        } else {
            false
        }
    }

    /// 重置累积时间
    pub fn reset_accumulated_time(&mut self) {
        self.accumulated_time = 0.0;
    }
}