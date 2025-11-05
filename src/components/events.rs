use bevy::ecs::message::Message;
use glam::IVec2;

/// 吃豆子事件
#[derive(Message)]
pub struct EatPelletEvent {
    pub position: IVec2,
}

