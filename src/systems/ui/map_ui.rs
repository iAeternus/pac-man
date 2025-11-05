use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    HALF, PELLET_COLOR, PELLET_RADIUS, PLAYER_COLOR, PLAYER_RADIUS, POWER_PELLET_COLOR,
    POWER_PELLET_RADIUS, Pellet, PelletType, Player, TILE_SIZE, WALL_COLOR, WALL_THICKNESS, Z_MAP,
    Z_PELLET, Z_PLAYER, Z_WALL,
    components::{MapData, TileType},
    is_power_pellet,
    ui::player_ui::{PelletUI, PlayerUI},
};

#[derive(Component)]
pub struct MapUI;

#[derive(Component)]
pub struct WallUI;

/// 绘制地图UI
pub fn setup_map_ui(mut commands: Commands, map_data: Res<MapData>) {
    // (offset_x, offset_y)为地图左上角点
    let offset_x = -((map_data.width as f32) * TILE_SIZE) / 2.0;
    let offset_y = ((map_data.height as f32) * TILE_SIZE) / 2.0;

    // 根节点
    let root = commands
        .spawn((
            Transform::from_xyz(0.0, 0.0, Z_MAP),
            GlobalTransform::default(),
            Visibility::default(),
            MapUI,
        ))
        .id();

    // 绘制地图
    let mut lines = vec![];
    for y in 0..map_data.height {
        for x in 0..map_data.width {
            let tile = map_data.get(x, y);
            // (px, py)为单元格左上角点
            let px = offset_x + x as f32 * TILE_SIZE;
            let py = offset_y - y as f32 * TILE_SIZE;

            match tile {
                TileType::Wall => {
                    calc_lines(&mut lines, &map_data, x, y, px, py);
                    // 绘制线段
                    for (a, b) in &lines {
                        commands.entity(root).with_children(|parent| {
                            let line = shapes::Line(*a, *b);
                            parent.spawn((
                                ShapeBuilder::with(&line)
                                    .stroke(Stroke::new(WALL_COLOR, WALL_THICKNESS))
                                    .build(),
                                Transform::from_xyz(0.0, 0.0, Z_WALL),
                                WallUI,
                            ));
                        });
                    }
                }
                TileType::Pellet => {
                    // 判断豆子类型
                    let (pellet, pellet_radius, pellet_color) = get_pellet_info(x, y);
                    // 绘制豆子
                    let circle = shapes::Circle {
                        radius: pellet_radius,
                        center: Vec2::new(px + HALF, py - HALF),
                    };
                    commands.entity(root).with_children(|parent| {
                        parent.spawn((
                            ShapeBuilder::with(&circle)
                                .fill(Fill::color(pellet_color))
                                .build(),
                            Transform::from_xyz(0.0, 0.0, Z_PELLET),
                            PelletUI,
                            pellet,
                        ));
                    });
                }
                TileType::Player => {
                    let circle = shapes::Circle {
                        radius: PLAYER_RADIUS,
                        center: Vec2::new(px + HALF, py - HALF),
                    };
                    commands.spawn((
                        ShapeBuilder::with(&circle)
                            .fill(Fill::color(PLAYER_COLOR))
                            .build(),
                        Transform::from_xyz(0.0, 0.0, Z_PLAYER),
                        PlayerUI,
                        Player::new(x as i32, y as i32),
                    ));
                }
                _ => {}
            }
        }
    }
}

fn calc_lines(
    lines: &mut Vec<(Vec2, Vec2)>,
    map_data: &MapData,
    x: usize,
    y: usize,
    px: f32,
    py: f32,
) {
    lines.clear();
    let top = y == 0 || !map_data.is_wall(x, y - 1);
    let bottom = y == map_data.height - 1 || !map_data.is_wall(x, y + 1);
    let left = x == 0 || !map_data.is_wall(x - 1, y);
    let right = x == map_data.width - 1 || !map_data.is_wall(x + 1, y);

    if top {
        lines.push((Vec2::new(px, py), Vec2::new(px + TILE_SIZE, py)));
    }
    if bottom {
        lines.push((
            Vec2::new(px, py - TILE_SIZE),
            Vec2::new(px + TILE_SIZE, py - TILE_SIZE),
        ));
    }
    if left {
        lines.push((Vec2::new(px, py), Vec2::new(px, py - TILE_SIZE)));
    }
    if right {
        lines.push((
            Vec2::new(px + TILE_SIZE, py),
            Vec2::new(px + TILE_SIZE, py - TILE_SIZE),
        ));
    }
}

/// 获取豆子渲染信息
fn get_pellet_info(x: usize, y: usize) -> (Pellet, f32, Color) {
    if is_power_pellet(x, y) {
        (
            Pellet::new(IVec2::new(x as i32, y as i32), PelletType::Power),
            POWER_PELLET_RADIUS,
            POWER_PELLET_COLOR,
        )
    } else {
        (
            Pellet::new(IVec2::new(x as i32, y as i32), PelletType::General),
            PELLET_RADIUS,
            PELLET_COLOR,
        )
    }
}
