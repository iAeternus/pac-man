use std::usize;

use bevy::prelude::*;

use crate::{
    MapData, Player, check_position,
    components::{EatPelletEvent, TileType},
};

/// 处理玩家输入
pub fn handle_player_input(
    mut query: Query<&mut Player>,
    keyboard: Res<ButtonInput<KeyCode>>,
    map_data: Res<MapData>,
) {
    for mut player in &mut query {
        let mut next = player.tile_pos;

        if keyboard.just_pressed(KeyCode::KeyW) {
            next.y -= 1;
        }
        if keyboard.just_pressed(KeyCode::KeyS) {
            next.y += 1;
        }
        if keyboard.just_pressed(KeyCode::KeyA) {
            next.x -= 1;
        }
        if keyboard.just_pressed(KeyCode::KeyD) {
            next.x += 1;
        }

        // 检查是否是墙
        if check_position(next.y, next.x, map_data.height, map_data.width)
            && map_data.tiles[next.y as usize][next.x as usize] != TileType::Wall
        {
            player.tile_pos = next;
        }
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
            if map_data.tiles[new_pos.y as usize][new_pos.x as usize] == TileType::Pellet {
                map_data.tiles[new_pos.y as usize][new_pos.x as usize] = TileType::Empty;
                eat_evt.write(EatPelletEvent { position: new_pos });
            }
        }
    }
}
