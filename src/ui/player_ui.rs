use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    HALF, MapData, MapUI, PELLET_COLOR, PELLET_RADIUS, Player, TILE_SIZE,
    game::{EatPelletEvent, TileType},
};

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
        transform.translation.x = offset_x + player.tile_pos.x as f32 * TILE_SIZE + HALF;
        transform.translation.y = offset_y - player.tile_pos.y as f32 * TILE_SIZE - HALF;
        info!(
    "Player tile=({}, {}), world=({}, {})",
    player.tile_pos.x, player.tile_pos.y,
    transform.translation.x, transform.translation.y
);

    }
}

/// 动态生成新豆子
pub fn spawn_new_pellet(
    mut commands: Commands,
    mut reader: MessageReader<EatPelletEvent>,
    map_query: Query<Entity, With<MapUI>>,
    map_data: Res<MapData>,
) {
    let Ok(root) = map_query.single() else {
        warn!("No MapUI root entity found!");
        return;
    };

    let offset_x = -((map_data.width as f32) * TILE_SIZE) / 2.0;
    let offset_y = ((map_data.height as f32) * TILE_SIZE) / 2.0;
    for _ in reader.read() {
        for y in 0..map_data.height {
            for x in 0..map_data.width {
                if map_data.tiles[y][x] == TileType::Pellet {
                    let px = offset_x + x as f32 * TILE_SIZE;
                    let py = offset_y - y as f32 * TILE_SIZE;
                    let circle = shapes::Circle {
                        radius: PELLET_RADIUS,
                        center: Vec2::new(px + HALF, py - HALF),
                    };
                    commands.entity(root).with_children(|parent| {
                        parent.spawn((
                            ShapeBuilder::with(&circle)
                                .fill(Fill::color(PELLET_COLOR))
                                .build(),
                            Transform::from_xyz(0.0, 0.0, 2.0),
                            PelletUI,
                        ));
                    });
                }
            }
        }
    }
}
