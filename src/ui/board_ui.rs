use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    MapDataResource,
    game::{MapData, TileType},
};

const TILE_SIZE: f32 = 20.0;
const HALF: f32 = TILE_SIZE / 2.0;

#[derive(Component)]
pub struct MapUI;

/// 绘制地图UI
pub fn setup_map_ui(mut commands: Commands, map_res: Res<MapDataResource>) {
    let map_data = &map_res.0;

    // 地图参数
    let wall_color = Color::srgb(0.0, 0.6, 1.0); // 蓝色墙
    let pellet_color = Color::WHITE; // 白色豆子
    let wall_thickness = 1.0_f32;
    let pellet_radius = 2.0_f32;

    // (offset_x, offset_y)为地图左上角点
    let offset_x = -((map_data.width as f32) * TILE_SIZE) / 2.0;
    let offset_y = ((map_data.height as f32) * TILE_SIZE) / 2.0;

    // 根节点
    let root = commands
        .spawn((
            Transform::from_xyz(0.0, 0.0, 0.0),
            GlobalTransform::default(),
            Visibility::default(),
            MapUI,
        ))
        .id();

    // 绘制地图
    let mut lines = vec![];
    for y in 0..map_data.height {
        for x in 0..map_data.width {
            let tile = map_data.tiles[y][x];
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
                                    .stroke(Stroke::new(wall_color, wall_thickness))
                                    .build(),
                                Transform::from_xyz(0.0, 0.0, 1.0),
                            ));
                        });
                    }
                }
                TileType::Pellet => {
                    let circle = shapes::Circle {
                        radius: pellet_radius,
                        center: Vec2::new(px + HALF, py - HALF),
                    };
                    commands.entity(root).with_children(|parent| {
                        parent.spawn((
                            ShapeBuilder::with(&circle)
                                .fill(Fill::color(pellet_color))
                                .build(),
                            Transform::from_xyz(0.0, 0.0, 2.0),
                        ));
                    });
                }
                TileType::Player | TileType::Ghost | TileType::Empty => {}
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
    let top = y == 0 || map_data.tiles[y - 1][x] != TileType::Wall;
    let bottom = y == map_data.height - 1 || map_data.tiles[y + 1][x] != TileType::Wall;
    let left = x == 0 || map_data.tiles[y][x - 1] != TileType::Wall;
    let right = x == map_data.width - 1 || map_data.tiles[y][x + 1] != TileType::Wall;

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
