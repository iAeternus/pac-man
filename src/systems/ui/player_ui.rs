use bevy::prelude::*;

use crate::{HALF, MapData, Player, TILE_SIZE, Z_PLAYER};

#[derive(Component)]
pub struct PlayerUI;

#[derive(Component)]
pub struct PelletUI;

/// 同步玩家实体与逻辑位置
pub fn sync_player_ui(
    mut player_query: Query<(&Player, &mut Transform), With<PlayerUI>>,
    map_data: Res<MapData>,
) {
    let offset_x = -((map_data.width as f32) * TILE_SIZE) / 2.0;
    let offset_y = ((map_data.height as f32) * TILE_SIZE) / 2.0;
    for (player, mut transform) in &mut player_query {
        let world_x = offset_x + player.tile_pos.x as f32 * TILE_SIZE + HALF;
        let world_y = offset_y - player.tile_pos.y as f32 * TILE_SIZE - HALF;
        // 不要问这个偏移量是怎么来的，问就是硬数的
        transform.translation = Vec3::new(
            world_x + 6.0 * TILE_SIZE + HALF,
            world_y + 8.0 * TILE_SIZE,
            Z_PLAYER,
        );
    }
}
