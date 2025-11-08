use bevy::{color::Color, ecs::component::Component};
use glam::IVec2;
use rand::Rng;
use std::collections::{BinaryHeap, HashMap};

use crate::{
    BLINKY_COLOR, CLYDE_COLOR, INKY_COLOR, MapData, Movement, PINKY_COLOR, TryMove, check_position,
};

/// 幽灵移动状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GhostMode {
    /// 追捕模式: 正常追捕玩家
    Chase,
    /// 分散模式: 在固定区域巡逻
    Scatter,
    /// 恐惧模式: 被能量豆影响，可以被吃掉
    Frightened,
    /// 返回基地: 被吃掉后返回重生点
    ReturnToBase,
}

/// 幽灵类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GhostType {
    Blinky, // 红色: 激进追捕
    Pinky,  // 粉色: 预判拦截
    Inky,   // 蓝色: 配合包抄
    Clyde,  // 橙色: 距离近时逃脱，距离远时追逐
}

/// A*算法节点
#[derive(Debug, Clone, PartialEq)]
struct Node {
    position: IVec2,
    f_cost: i32, // f = g + h
    g_cost: i32, // 从起点到当前节点的代价
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // 最小堆，所以比较顺序反过来
        other.f_cost.partial_cmp(&self.f_cost)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_cost.cmp(&self.f_cost)
    }
}

/// 幽灵
#[derive(Debug, Component)]
pub struct Ghost {
    /// 当前地图格坐标
    pub tile_pos: IVec2,
    /// 幽灵类型
    pub ghost_type: GhostType,
    /// 初始重生位置
    pub spawn_pos: IVec2,
    /// 目标位置
    pub target_pos: IVec2,
    /// 状态
    pub mode: GhostMode,
    /// 恐惧状态剩余时间
    pub frightened_time: f32,
    /// 分散模式计时器
    pub scatter_timer: f32,
    /// 是否在基地中
    pub in_house: bool,
    /// 离开基地的延迟
    pub house_delay: f32,
    /// 移动组件
    pub movement: Movement,
    /// 是否在交叉路口需要重新选择方向
    pub at_intersection: bool,
    /// 上次移动方向
    pub last_move_dir: IVec2,
    /// 模式切换计时器
    pub mode_switch_timer: f32,
    /// 当前模式持续时间
    pub current_mode_duration: f32,
    /// 路径缓存
    path_cache: Option<Vec<IVec2>>,
}

impl Ghost {
    pub fn new(x: i32, y: i32, ghost_type: GhostType) -> Self {
        Self {
            tile_pos: IVec2::new(x, y),
            ghost_type,
            spawn_pos: IVec2::new(x, y),
            target_pos: IVec2::new(x, y),
            mode: GhostMode::Scatter,
            frightened_time: 0.0,
            scatter_timer: 0.0,
            in_house: true,
            house_delay: match ghost_type {
                GhostType::Blinky => 0.0,
                GhostType::Pinky => 5.0,
                GhostType::Inky => 10.0,
                GhostType::Clyde => 15.0,
            },
            movement: Movement::new(4.0, true),
            at_intersection: false,
            last_move_dir: IVec2::ZERO,
            mode_switch_timer: 0.0,
            current_mode_duration: 7.0, // 初始分散模式7秒
            path_cache: None,
        }
    }

    pub fn set_frightened(&mut self, duration: f32) {
        if self.mode != GhostMode::ReturnToBase {
            self.mode = GhostMode::Frightened;
            self.frightened_time = duration;
            // 恐惧状态下速度减半，但方向保持不变
            self.movement.speed = 2.0;
            // 清除路径缓存
            self.path_cache = None;
        }
    }

    /// 被吃掉，返回基地
    pub fn eaten(&mut self) {
        self.mode = GhostMode::ReturnToBase;
        // 注意：这里不立即重置位置，让幽灵自己移动回基地
        // 返回基地时高速移动
        self.movement.speed = 8.0;
        // 清除路径缓存，重新计算回基地的路径
        self.path_cache = None;
    }

    /// 离开基地
    pub fn try_leave_house(&mut self) {
        if self.in_house && self.house_delay <= 0.0 {
            self.in_house = false;
            self.mode = GhostMode::Scatter;
            self.movement.speed = 4.0; // 恢复正常速度
            self.mode_switch_timer = 0.0;
            self.current_mode_duration = 7.0; // 分散模式7秒
        }
    }

    /// A*路径查找算法
    fn find_path(&self, start: IVec2, target: IVec2, map_data: &MapData) -> Option<Vec<IVec2>> {
        if start == target {
            return Some(vec![start]);
        }

        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        let mut f_score = HashMap::new();

        // 启发式函数：曼哈顿距离
        let heuristic = |a: IVec2, b: IVec2| -> i32 { (a.x - b.x).abs() + (a.y - b.y).abs() };

        g_score.insert(start, 0);
        f_score.insert(start, heuristic(start, target));

        open_set.push(Node {
            position: start,
            f_cost: f_score[&start],
            g_cost: 0,
        });

        while let Some(current) = open_set.pop() {
            if current.position == target {
                // 重建路径
                let mut path = vec![current.position];
                let mut current_pos = current.position;

                while let Some(&prev) = came_from.get(&current_pos) {
                    path.push(prev);
                    current_pos = prev;
                }
                path.reverse();
                return Some(path);
            }

            let neighbors = vec![
                current.position + IVec2::new(1, 0),
                current.position + IVec2::new(-1, 0),
                current.position + IVec2::new(0, 1),
                current.position + IVec2::new(0, -1),
            ];

            for neighbor in neighbors {
                // 检查边界和墙壁
                if !check_position(neighbor.x, neighbor.y, map_data.width, map_data.height)
                    || map_data.is_wall(neighbor.x as usize, neighbor.y as usize)
                {
                    continue;
                }

                let tentative_g_score = g_score[&current.position] + 1;

                if !g_score.contains_key(&neighbor) || tentative_g_score < g_score[&neighbor] {
                    came_from.insert(neighbor, current.position);
                    g_score.insert(neighbor, tentative_g_score);
                    let f_cost = tentative_g_score + heuristic(neighbor, target);
                    f_score.insert(neighbor, f_cost);

                    open_set.push(Node {
                        position: neighbor,
                        f_cost,
                        g_cost: tentative_g_score,
                    });
                }
            }
        }

        None // 没有找到路径
    }

    /// 计算 target_pos
    pub fn calc_target(
        &self,
        player_pos: IVec2,
        player_dir: IVec2,
        blinky_pos: IVec2,
        map_data: &MapData,
        _rng: &mut impl Rng,
    ) -> IVec2 {
        match self.mode {
            GhostMode::ReturnToBase => self.spawn_pos,
            GhostMode::Frightened => {
                // 恐惧模式下随机移动，但避免立即回头
                let possible_dirs = self.get_possible_direction(map_data, true);
                if possible_dirs.is_empty() {
                    return player_pos;
                }

                // 在原版吃豆人中，恐惧模式下的幽灵会随机选择方向
                // 但不会立即180度回头，除非没有其他选择
                let non_reverse_dirs: Vec<IVec2> = possible_dirs
                    .iter()
                    .filter(|&&dir| dir != -self.last_move_dir)
                    .cloned()
                    .collect();

                if !non_reverse_dirs.is_empty() {
                    // 随机选择一个非回头方向
                    let random_idx = _rng.random_range(0..non_reverse_dirs.len());
                    self.tile_pos + non_reverse_dirs[random_idx] * 4
                } else {
                    // 只有回头路可走
                    let random_idx = _rng.random_range(0..possible_dirs.len());
                    self.tile_pos + possible_dirs[random_idx] * 4
                }
            }
            GhostMode::Scatter => {
                // 分散模式下每个幽灵有自己的角落
                match self.ghost_type {
                    GhostType::Blinky => {
                        IVec2::new(map_data.width as i32 - 2, map_data.height as i32 - 2)
                    }
                    GhostType::Pinky => IVec2::new(1, map_data.height as i32 - 2),
                    GhostType::Inky => IVec2::new(map_data.width as i32 - 2, 1),
                    GhostType::Clyde => IVec2::new(1, 1),
                }
            }
            GhostMode::Chase => {
                // 追赶模式下每个幽灵有不同的策略
                match self.ghost_type {
                    GhostType::Blinky => {
                        // Blinky: 直接追击玩家当前位置
                        player_pos
                    }
                    GhostType::Pinky => {
                        // Pinky: 预判玩家前方4格，但需要处理穿墙问题
                        let target = player_pos + player_dir * 4;
                        // 确保目标位置在地图范围内
                        IVec2::new(
                            target.x.clamp(0, map_data.width as i32 - 1),
                            target.y.clamp(0, map_data.height as i32 - 1),
                        )
                    }
                    GhostType::Inky => {
                        // Inky: 基于玩家位置和Blinky位置的复杂计算
                        // 计算玩家前方2格的位置
                        let player_ahead = player_pos + player_dir * 2;
                        // 计算从Blinky到玩家前方位置的向量
                        let blinky_to_player_ahead = player_ahead - blinky_pos;
                        // 目标位置是玩家前方位置加上这个向量的两倍
                        let target = player_ahead + blinky_to_player_ahead * 2;
                        IVec2::new(
                            target.x.clamp(0, map_data.width as i32 - 1),
                            target.y.clamp(0, map_data.height as i32 - 1),
                        )
                    }
                    GhostType::Clyde => {
                        // Clyde: 距离近时逃回角落，距离远时追击
                        let distance_sq = (self.tile_pos - player_pos).as_vec2().length_squared();
                        if distance_sq < 64.0 {
                            // 8格距离的平方
                            // 逃回左下角
                            IVec2::new(1, 1)
                        } else {
                            // 追击玩家
                            player_pos
                        }
                    }
                }
            }
        }
    }

    /// 获取可能的移动方向
    pub fn get_possible_direction(&self, map_data: &MapData, allow_reverse: bool) -> Vec<IVec2> {
        let dirs = vec![
            IVec2::new(1, 0),  // 右
            IVec2::new(0, 1),  // 下
            IVec2::new(-1, 0), // 左
            IVec2::new(0, -1), // 上
        ];

        dirs.into_iter()
            .filter(|&dir| {
                // 在非恐惧模式下，幽灵不能立即180度回头（原版吃豆人行为）
                if !allow_reverse
                    && self.mode != GhostMode::Frightened
                    && dir == -self.last_move_dir
                {
                    return false;
                }

                let new_pos = self.tile_pos + dir;
                check_position(new_pos.x, new_pos.y, map_data.width, map_data.height)
                    && !map_data.is_wall(new_pos.x as usize, new_pos.y as usize)
            })
            .collect()
    }

    /// 检查是否在交叉路口
    pub fn check_intersection(&self, map_data: &MapData) -> bool {
        let possible_dirs = self.get_possible_direction(map_data, true);
        possible_dirs.len() > 2
            || (possible_dirs.len() == 2 && !possible_dirs.contains(&-self.last_move_dir))
    }

    /// 选择最佳方向
    pub fn choose_best_direction(
        &mut self,
        target_pos: IVec2,
        possible_dirs: &[IVec2],
        map_data: &MapData,
        rng: &mut impl Rng,
    ) -> Option<IVec2> {
        if possible_dirs.is_empty() {
            return None;
        }

        // 恐惧模式下随机选择，但避免立即回头
        if self.mode == GhostMode::Frightened {
            let non_reverse_dirs: Vec<IVec2> = possible_dirs
                .iter()
                .filter(|&&dir| dir != -self.last_move_dir)
                .cloned()
                .collect();

            if !non_reverse_dirs.is_empty() {
                let random_idx = rng.random_range(0..non_reverse_dirs.len());
                return Some(non_reverse_dirs[random_idx]);
            }
            // 只有回头路可走
            let random_idx = rng.random_range(0..possible_dirs.len());
            return Some(possible_dirs[random_idx]);
        }

        // 返回基地时使用A*算法
        if self.mode == GhostMode::ReturnToBase {
            // 使用缓存的路径或计算新路径
            if self.path_cache.is_none() {
                self.path_cache = self.find_path(self.tile_pos, target_pos, map_data);
            }

            if let Some(ref path) = self.path_cache {
                if path.len() > 1 {
                    let next_pos = path[1]; // 路径中的下一个位置
                    let direction = next_pos - self.tile_pos;

                    // 如果这个方向是可行的，就选择它
                    if possible_dirs.contains(&direction) {
                        return Some(direction);
                    }
                }
            }

            // 如果A*失败，回退到最短距离
            return possible_dirs
                .iter()
                .min_by_key(|&&dir| {
                    let new_pos = self.tile_pos + dir;
                    (new_pos - target_pos).length_squared()
                })
                .copied();
        }

        // 对于追逐和分散模式，只在复杂情况下使用A*
        if possible_dirs.len() > 2 {
            // 使用A*找到最佳路径
            if let Some(path) = self.find_path(self.tile_pos, target_pos, map_data) {
                if path.len() > 1 {
                    let next_pos = path[1];
                    let direction = next_pos - self.tile_pos;

                    if possible_dirs.contains(&direction) {
                        return Some(direction);
                    }
                }
            }
        }

        // 简单情况下使用曼哈顿距离
        possible_dirs
            .iter()
            .min_by_key(|&&dir| {
                let new_pos = self.tile_pos + dir;
                let dx = (new_pos.x - target_pos.x).abs();
                let dy = (new_pos.y - target_pos.y).abs();
                dx + dy // 曼哈顿距离
            })
            .copied()
    }

    /// 更新状态计时器
    pub fn update_timers(&mut self, delta_secs: f32) {
        // 更新离开基地的延迟
        if self.in_house {
            self.house_delay -= delta_secs;
            self.try_leave_house();
            return; // 在基地内不更新其他计时器
        }

        // 更新模式切换计时器
        self.mode_switch_timer += delta_secs;

        match self.mode {
            GhostMode::Frightened => {
                self.frightened_time -= delta_secs;
                if self.frightened_time <= 0.0 {
                    // 恐惧状态结束，恢复到之前的模式
                    self.mode = GhostMode::Chase;
                    self.movement.speed = 4.0;
                    self.mode_switch_timer = 0.0;
                    self.current_mode_duration = 20.0; // 追逐模式20秒
                    // 清除路径缓存
                    self.path_cache = None;
                }
            }
            GhostMode::Scatter | GhostMode::Chase => {
                // 原版吃豆人的模式切换序列: 散射7秒 → 追逐20秒 → 散射7秒 → 追逐20秒 → 散射5秒 → 追逐20秒 → 散射5秒 → 永久追逐
                if self.mode_switch_timer >= self.current_mode_duration {
                    let old_mode = self.mode;
                    self.mode = if self.mode == GhostMode::Scatter {
                        GhostMode::Chase
                    } else {
                        GhostMode::Scatter
                    };

                    self.mode_switch_timer = 0.0;

                    // 只在模式真正改变时清除路径缓存
                    if old_mode != self.mode {
                        self.path_cache = None;
                    }

                    // 更新下一个模式的持续时间
                    self.current_mode_duration = match self.mode {
                        GhostMode::Scatter => {
                            // 散射模式持续时间递减
                            if self.scatter_timer < 14.0 {
                                // 第一次散射7秒，第二次散射7秒
                                7.0
                            } else if self.scatter_timer < 19.0 {
                                // 第三次散射5秒
                                5.0
                            } else {
                                // 第四次散射5秒后永久追逐
                                5.0
                            }
                        }
                        GhostMode::Chase => {
                            // 追逐模式固定20秒，直到第四次散射后变为永久追逐
                            if self.scatter_timer >= 24.0 {
                                f32::INFINITY // 永久追逐
                            } else {
                                20.0
                            }
                        }
                        _ => 20.0,
                    };

                    self.scatter_timer += self.current_mode_duration;
                }
            }
            GhostMode::ReturnToBase => {
                // 回到基地后恢复正常
                if self.tile_pos == self.spawn_pos {
                    self.mode = GhostMode::Chase;
                    self.movement.speed = 4.0;
                    self.mode_switch_timer = 0.0;
                    self.current_mode_duration = 20.0;
                    self.path_cache = None;
                }
            }
        }
    }

    /// 更新幽灵状态
    pub fn update(
        &mut self,
        player_pos: IVec2,
        player_dir: IVec2,
        blinky_pos: IVec2,
        map_data: &MapData,
        rng: &mut impl Rng,
        delta_secs: f32,
    ) {
        self.update_timers(delta_secs);

        // 只在交叉路口或需要时重新计算方向
        self.at_intersection = self.check_intersection(map_data);

        // 在以下情况下重新计算方向：
        // 1. 在交叉路口
        // 2. 当前没有移动方向
        // 3. 即将撞墙
        // 4. 目标位置改变
        let should_recalculate = self.at_intersection
            || self.movement.direction == IVec2::ZERO
            || self.will_hit_wall(map_data);

        if should_recalculate {
            let new_target = self.calc_target(player_pos, player_dir, blinky_pos, map_data, rng);

            // 如果目标位置改变，清除路径缓存
            if new_target != self.target_pos {
                self.target_pos = new_target;
                self.path_cache = None;
            }

            let possible_dirs = self.get_possible_direction(map_data, false);

            if let Some(new_dir) =
                self.choose_best_direction(self.target_pos, &possible_dirs, map_data, rng)
            {
                self.movement.direction = new_dir;
                self.last_move_dir = new_dir;
            }
        }
    }

    /// 检查是否会撞墙
    fn will_hit_wall(&self, map_data: &MapData) -> bool {
        if self.movement.direction == IVec2::ZERO {
            return false;
        }

        let next_pos = self.tile_pos + self.movement.direction;
        !check_position(next_pos.x, next_pos.y, map_data.width, map_data.height)
            || map_data.is_wall(next_pos.x as usize, next_pos.y as usize)
    }
}

impl TryMove for Ghost {
    fn try_move(&mut self, map_data: &MapData) -> Option<IVec2> {
        if self.movement.direction == IVec2::ZERO || !self.movement.is_moving {
            return None;
        }

        let new_pos = self.tile_pos + self.movement.direction;
        if !map_data.is_valid_position(new_pos.x, new_pos.y)
            || map_data.is_wall(new_pos.x as usize, new_pos.y as usize)
        {
            return None;
        }

        Some(new_pos)
    }
}

impl From<GhostType> for Color {
    fn from(ghost_type: GhostType) -> Self {
        match ghost_type {
            GhostType::Blinky => BLINKY_COLOR,
            GhostType::Pinky => PINKY_COLOR,
            GhostType::Inky => INKY_COLOR,
            GhostType::Clyde => CLYDE_COLOR,
        }
    }
}
