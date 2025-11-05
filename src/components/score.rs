use bevy::ecs::resource::Resource;

/// 分数
#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

impl Score {
    /// 吃豆分数 + 1
    pub fn plus_one(&mut self) {
        self.value += 1;
    }
}
