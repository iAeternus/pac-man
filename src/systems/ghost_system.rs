use bevy::prelude::*;

use crate::{Ghost, GhostType, MapData, Player, TryMove};

/// 幽灵AI系统
pub fn ghost_ai_system(
    mut ghost_query: Query<&mut Ghost>,
    player_query: Query<&Player>,
    map_data: Res<MapData>,
    time: Res<Time>,
) {
    if let Ok(player) = player_query.single() {
        // 获取Blinky的位置（用于Inky的计算）
        let blinky_pos = ghost_query
            .iter()
            .find(|ghost| ghost.ghost_type == GhostType::Blinky)
            .map(|ghost| ghost.tile_pos)
            .unwrap_or(player.tile_pos);

        // 为每个幽灵创建独立的RNG实例
        let mut rng = rand::rng();

        for mut ghost in ghost_query.iter_mut() {
            ghost.update(
                player.tile_pos,
                player.movement.direction,
                blinky_pos,
                &map_data,
                &mut rng,
                time.delta_secs(),
            );
        }
    }
}

/// 幽灵移动系统
pub fn ghost_move_system(
    mut ghost_query: Query<&mut Ghost>,
    map_data: Res<MapData>,
    time: Res<Time>,
) {
    for mut ghost in ghost_query.iter_mut() {
        while ghost.movement.update(time.delta_secs()) {
            if let Some(new_pos) = ghost.try_move(&map_data) {
                // if new_pos == IVec2::new(14, 0) {

                // }
                // if new_pos == IVec2::new(14, 27) {

                // }
                ghost.tile_pos = new_pos;
            }
        }
    }
}
