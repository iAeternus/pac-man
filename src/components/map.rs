use std::{fs, path::Path};

use bevy::ecs::resource::Resource;

/// 单元格类型
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    Empty,
    Wall,
    Pellet,
    Player,
    Ghost,
}

/// 地图数据
/// 注意：只有数组访问是 tiles[y][x]，其他一律为 (x, y)
#[derive(Debug, Resource)]
pub struct MapData {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<TileType>>,
}

impl MapData {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![vec![TileType::Empty; width]; height],
        }
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        self.tiles[y][x] == TileType::Wall
    }

    pub fn is_pellet(&self, x: usize, y: usize) -> bool {
        self.tiles[y][x] == TileType::Pellet
    }

    pub fn is_player(&self, x: usize, y: usize) -> bool {
        self.tiles[y][x] == TileType::Player
    }

    pub fn is_ghost(&self, x: usize, y: usize) -> bool {
        self.tiles[y][x] == TileType::Ghost
    }

    pub fn set(&mut self, x: usize, y: usize, tile_type: TileType) {
        self.tiles[y][x] = tile_type;
    }

    pub fn get(&self, x: usize, y: usize) -> TileType {
        self.tiles[y][x]
    }
}

/// 地图加载器
pub trait MapLoader {
    fn load_map(&self, path: &Path) -> anyhow::Result<MapData>;
}

/// 文本地图加载
pub struct TextMapLoader;

impl MapLoader for TextMapLoader {
    fn load_map(&self, path: &Path) -> anyhow::Result<MapData> {
        let lines = fs::read_to_string(path)?
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();
        let height = lines.len();
        let width = lines.get(0).map_or(0, |l| l.len());
        let mut map = MapData::new(width, height);

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                map.tiles[y][x] = match ch {
                    '#' => TileType::Wall,
                    '.' => TileType::Pellet,
                    'P' => TileType::Player,
                    'G' => TileType::Ghost,
                    _ => TileType::Empty,
                }
            }
        }
        Ok(map)
    }
}

/// 检查当前坐标是否合法
pub fn check_position(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && (x as usize) < width && y >= 0 && (y as usize) < height
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATH: &'static str = "assets\\map\\pacman.map";

    #[test]
    fn test_text_map_loader() -> anyhow::Result<()> {
        let map_loader = TextMapLoader;
        let map_data = map_loader.load_map(Path::new(PATH))?;

        assert_eq!(map_data.height, 31);
        assert_eq!(map_data.width, 28);
        Ok(())
    }
}
