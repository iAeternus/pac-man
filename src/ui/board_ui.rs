use std::path::Path;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::game::{MapLoader, TextMapLoader, TileType};

const TILE_SIZE: f32 = 20.0;
const HALF: f32 = TILE_SIZE / 2.0;
const MAP_PATH: &'static str = "assets\\map\\pacman.map";

#[derive(Component)]
pub struct MapUI;

/// 绘制地图UI
pub fn setup_map_ui(mut commands: Commands) {
    // 加载地图
    let map_path = Path::new(MAP_PATH);
    let map_loader = TextMapLoader;
    let map_data = map_loader.load_map(map_path).expect("Load map failed.");

    // 地图参数
    let wall_color = Color::srgb(0.0, 0.6, 1.0); // 亮蓝
    let pellet_color = Color::srgb(1.0, 1.0, 0.0); // 黄色豆子
    let wall_thickness = 2.0_f32;
    let pellet_radius = 3.0_f32;

    // 地图居中
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
    for y in 0..map_data.height {
        for x in 0..map_data.width {
            let tile = map_data.tiles[y][x];
            let px = offset_x + x as f32 * TILE_SIZE;
            let py = offset_y - y as f32 * TILE_SIZE;

            match tile {
                TileType::Wall => {
                    let top = y == 0 || map_data.tiles[y - 1][x] != TileType::Wall;
                    let bottom =
                        y == map_data.height - 1 || map_data.tiles[y + 1][x] != TileType::Wall;
                    let left = x == 0 || map_data.tiles[y][x - 1] != TileType::Wall;
                    let right =
                        x == map_data.width - 1 || map_data.tiles[y][x + 1] != TileType::Wall;

                    let mut lines = vec![];
                    if top {
                        lines.push((
                            Vec2::new(px, py + HALF),
                            Vec2::new(px + TILE_SIZE, py + HALF),
                        ));
                    }
                    if bottom {
                        lines.push((
                            Vec2::new(px, py - HALF),
                            Vec2::new(px + TILE_SIZE, py - HALF),
                        ));
                    }
                    if left {
                        lines.push((
                            Vec2::new(px, py - HALF),
                            Vec2::new(px, py + HALF),
                        ));
                    }
                    if right {
                        lines.push((
                            Vec2::new(px + TILE_SIZE, py - HALF),
                            Vec2::new(px + TILE_SIZE, py + HALF),
                        ));
                    }

                    // 绘制线段
                    for (a, b) in lines {
                        commands.entity(root).with_children(|parent| {
                            let line = shapes::Line(a, b);
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
                        center: Vec2::new(px + HALF, py),
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
