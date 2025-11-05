use bevy::prelude::*;

use crate::{
    MapData, Player,
    game::{EatPelletEvent, TileType},
};

/// 处理玩家输入
pub fn handle_player_input(mut query: Query<&mut Player>, keyboard: Res<ButtonInput<KeyCode>>) {
    for mut player in &mut query {
        let mut dir = IVec2::ZERO;
        if keyboard.pressed(KeyCode::KeyW) {
            dir.y -= 1; // 上
        }
        if keyboard.pressed(KeyCode::KeyS) {
            dir.y += 1; // 下
        }
        if keyboard.pressed(KeyCode::KeyA) {
            dir.x -= 1; // 左
        }
        if keyboard.pressed(KeyCode::KeyD) {
            dir.x += 1; // 右
        }
        player.set_direction(dir);
    }
}

/// 更新玩家
pub fn player_update(
    mut query: Query<&mut Player>,
    mut map_data: ResMut<MapData>,
    mut eat_evt: MessageWriter<EatPelletEvent>,
) {
    for mut player in &mut query {
        if let Some(new_pos) = player.try_move(&map_data.tiles) {
            player.tile_pos = new_pos;
            info!(
                "tile_pos: {:?}, dir: {:?}",
                player.tile_pos, player.direction
            );

            if map_data.tiles[new_pos.y as usize][new_pos.x as usize] == TileType::Pellet {
                map_data.tiles[new_pos.y as usize][new_pos.x as usize] = TileType::Empty;
                eat_evt.write(EatPelletEvent { position: new_pos });
            }
        }
    }
}
