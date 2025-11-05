use std::path::Path;

use bevy::ecs::system::Commands;

use crate::{MAP_PATH, MapLoader, TextMapLoader};

/// 加载地图数据，全局只加载一次
pub fn load_map_data(mut commands: Commands) {
    let loader = TextMapLoader;
    let map_path = Path::new(MAP_PATH);
    let map_data = loader.load_map(map_path).expect("Failed to load map");

    commands.insert_resource(map_data);
}
