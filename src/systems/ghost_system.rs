use bevy::prelude::*;

use crate::{Ghost, GhostType, MapData, Player, TryMove};

/// 幽灵AI系统
pub fn ghost_ai_system(
    mut ghost_query: Query<&mut Ghost>,
    player_query: Query<&mut Player>,
    map_data: Res<MapData>,
    time: Res<Time>,
) {
    if let Ok(player) = player_query.single() {
        let blinky_pos = ghost_query
            .iter()
            .find(|&ghost| ghost.ghost_type == GhostType::Blinky)
            .map(|ghost| ghost.tile_pos)
            .unwrap_or(player.tile_pos);
        let mut rng = rand::rng();
        for mut ghost in &mut ghost_query {
            // 更新幽灵计时器
            ghost.update_timers(time.delta_secs());

            if !ghost.movement.is_moving {
                continue;
            }

            // 计算目标位置
            let target_pos = ghost.calc_target(
                player.tile_pos,
                player.movement.direction,
                blinky_pos,
                &map_data,
                &mut rng,
            );

            // 获取可能的移动方向
            let possible_dirs = ghost.get_possible_direction(&map_data);

            // 选择最佳方向
            if let Some(best_dir) =
                ghost.choose_best_direction(target_pos, &possible_dirs, &mut rng)
            {
                ghost.movement.set_direction(best_dir);
                ghost.target_pos = target_pos;
            }
        }
    }
}

/// 幽灵移动系统
pub fn ghost_move_system(mut ghost_query: Query<&mut Ghost>, map_data: Res<MapData>, time: Res<Time>) {
    for mut ghost in &mut ghost_query {
        while ghost.movement.update(time.delta_secs()) {
            // 执行移动
            if let Some(new_pos) = ghost.try_move(&map_data) {
                ghost.tile_pos = new_pos;
            }
        }
    }
}
