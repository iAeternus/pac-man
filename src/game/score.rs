use bevy::ecs::resource::Resource;

/// 分数
#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}