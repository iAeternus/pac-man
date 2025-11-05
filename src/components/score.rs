use bevy::ecs::resource::Resource;

/// 分数
#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

impl Score {
    pub fn new(val: u32) -> Self {
        Self { value: val }
    }
}
