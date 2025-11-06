use bevy::prelude::*;

use crate::{EatPelletEvent, Pellet, PelletType, PelletUI, Score};

/// 处理吃豆事件
pub fn handle_eat_pellet_message(
    mut commands: Commands,
    mut reader: MessageReader<EatPelletEvent>,
    pellet_query: Query<(Entity, &Pellet), With<PelletUI>>,
    mut score: ResMut<Score>,
) {
    for evt in reader.read() {
        remove_pellet(&mut commands, &pellet_query, evt.position);
        update_score(&mut score, &evt.pellet_type);

        // TODO: 播放音效
        // TODO: 触发特效
        // TODO: 检查是否触发特殊事件（如能量豆）
    }
}

/// 移除豆子实体
fn remove_pellet(
    commands: &mut Commands,
    pellet_query: &Query<(Entity, &Pellet), With<PelletUI>>,
    position: IVec2,
) {
    for (entity, pellet) in pellet_query.iter() {
        if pellet.position == position {
            commands.entity(entity).despawn();
            break;
        }
    }
}

/// 更新分数
fn update_score(score: &mut ResMut<Score>, pellet_type: &PelletType) {
    match pellet_type {
        PelletType::General => {
            score.value += 1; // 普通豆子加1分
        }
        PelletType::Power => {
            score.value += 10; // 能量豆加10分
            // TODO: 触发幽灵变蓝等特殊效果
        }
    }
}
