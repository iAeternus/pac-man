use std::usize;

use bevy::prelude::*;

use crate::{
    MapData, Player, TryMove, check_position,
    components::{EatPelletEvent, TileType},
};

/// 处理玩家输入，设置玩家方向
pub fn handle_player_input(
    mut query: Query<&mut Player>,
    keyboard: Res<ButtonInput<KeyCode>>,
    map_data: Res<MapData>,
) {
    for mut player in &mut query {
        let mut dir = IVec2::ZERO;
        if keyboard.pressed(KeyCode::KeyW) {
            dir.y -= 1;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            dir.y += 1;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            dir.x -= 1;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            dir.x += 1;
        }

        // 检查目标位置是否可移动
        if dir != IVec2::ZERO {
            let next_pos = player.tile_pos + dir;
            if check_position(next_pos.x, next_pos.y, map_data.width, map_data.height)
                && !map_data.is_wall(next_pos.x as usize, next_pos.y as usize)
            {
                player.movement.set_direction(dir);
            }
        }
    }
}

/// 更新玩家
pub fn player_update(
    mut query: Query<&mut Player>,
    mut map_data: ResMut<MapData>,
    mut eat_evt: MessageWriter<EatPelletEvent>,
    time: Res<Time>,
) {
    for mut player in &mut query {
        while player.movement.update(time.delta_secs()) {
            // 执行移动
            if let Some(new_pos) = player.try_move(&map_data) {
                player.tile_pos = new_pos;
                if map_data.is_pellet(new_pos.x as usize, new_pos.y as usize) {
                    map_data.set(new_pos.x as usize, new_pos.y as usize, TileType::Empty);
                    eat_evt.write(EatPelletEvent::new(new_pos));
                }
            }
        }
    }
}
