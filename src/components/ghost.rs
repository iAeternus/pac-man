use bevy::{color::Color, ecs::component::Component};
use glam::IVec2;
use rand::Rng;

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
}

impl Ghost {
    pub fn new(x: i32, y: i32, ghost_type: GhostType) -> Self {
        // 给一个初始向上的方向
        let init_dir = IVec2::new(0, -1);

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
            // movement: Movement::new(4.0), // 速度比玩家稍慢
            movement: Movement {
                direction: init_dir,
                speed: 4.0,
                is_moving: true,
                accumulated_time: 0.0,
            },
        }
    }

    pub fn set_frightened(&mut self, duration: f32) {
        self.mode = GhostMode::Frightened;
        self.frightened_time = duration;
        // 恐惧状态下速度减半
        self.movement.speed /= 2.0;
    }

    /// 被吃掉，返回基地
    pub fn eaten(&mut self) {
        self.mode = GhostMode::ReturnToBase;
        self.tile_pos = self.spawn_pos;
        self.in_house = true;
        // 返回基地时高速移动
        self.movement.speed = 8.0;
    }

    /// 离开基地
    pub fn try_leave_house(&mut self) {
        if self.in_house && self.house_delay <= 0.0 {
            self.in_house = false;
            self.mode = GhostMode::Scatter;
            self.movement.speed = 4.0; // 恢复正常速度
        }
    }

    /// 计算 target_pos
    ///
    /// ## Params
    /// - player_pos: 玩家位置
    /// - player_dir: 玩家方向
    /// - blinky_pos: Blinky位置，用于Inky的特殊计算
    /// - map_data: 地图数据
    /// - rng: 随机数生成器
    pub fn calc_target(
        &self,
        player_pos: IVec2,
        player_dir: IVec2,
        blinky_pos: IVec2,
        map_data: &MapData,
        rng: &mut impl Rng,
    ) -> IVec2 {
        match self.mode {
            GhostMode::ReturnToBase => self.spawn_pos,
            GhostMode::Frightened => {
                // 恐惧模式下随机移动
                let possible_dirs = self.get_possible_direction(map_data);
                if possible_dirs.is_empty() {
                    return player_pos; // 备用目标
                }
                let random_idx = rng.random_range(0..possible_dirs.len()); // 随机选择一个方向
                let dis = rng.random_range(2..=4); // 随机选择一个距离
                self.tile_pos + possible_dirs[random_idx] * dis
            }
            GhostMode::Scatter => {
                // 分散模式下每个幽灵有自己的角落
                match self.ghost_type {
                    GhostType::Blinky => {
                        // 右上角
                        IVec2::new(map_data.width as i32 - 3, map_data.height as i32 - 3)
                    }
                    GhostType::Pinky => IVec2::new(2, map_data.height as i32 - 3), // 左上角
                    GhostType::Inky => IVec2::new(map_data.width as i32 - 3, 2),   // 右下角
                    GhostType::Clyde => IVec2::new(2, 2),                          // 左下角
                }
            }
            GhostMode::Chase => {
                // 追赶模式下每个幽灵有不同的策略
                match self.ghost_type {
                    GhostType::Blinky => player_dir,                 // 直接追击玩家
                    GhostType::Pinky => player_pos + player_dir * 4, // 预判玩家前方4格
                    GhostType::Inky => {
                        // 基于玩家位置和Blinky位置
                        let player_ahead = player_pos + player_dir * 2;
                        let blinky_to_player_ahead = player_ahead - blinky_pos;
                        player_ahead + blinky_to_player_ahead
                    }
                    GhostType::Clyde => {
                        // 距离近时逃离角落，距离远时追击
                        let dis_to_player = (self.tile_pos - player_pos).as_vec2().length();
                        if dis_to_player < 8.0 {
                            // 逃回角落
                            IVec2::new(2, 2)
                        } else {
                            // 追击玩家
                            player_pos
                        }
                    }
                }
            }
        }
    }

    /// 获取可能的移动方向，排除墙壁和反向
    pub fn get_possible_direction(&self, map_data: &MapData) -> Vec<IVec2> {
        let dirs = vec![
            IVec2::new(1, 0),
            IVec2::new(-1, 0),
            IVec2::new(0, 1),
            IVec2::new(0, -1),
        ];
        let rev_direction = -self.movement.direction;

        dirs.into_iter()
            .filter(|&dir| {
                // 不能走回头路（除非在恐惧模式或返回基地）
                if self.mode != GhostMode::Frightened
                    && self.mode == GhostMode::ReturnToBase
                    && dir == rev_direction
                {
                    return false;
                }

                let new_pos = self.tile_pos + dir;
                check_position(new_pos.x, new_pos.y, map_data.width, map_data.height)
                    && !map_data.is_wall(new_pos.x as usize, new_pos.y as usize)
            })
            .collect()
    }

    /// 选择最佳方向
    ///
    /// ## Params
    /// - target_pos: 目标位置
    /// - possible_dirs: 可能的方向集合
    /// - rng: 随机数生成器
    pub fn choose_best_direction(
        &self,
        target_pos: IVec2,
        possible_dirs: &Vec<IVec2>,
        rng: &mut impl Rng,
    ) -> Option<IVec2> {
        if possible_dirs.is_empty() {
            return None;
        }

        // 恐惧模式下随机选择
        if self.mode == GhostMode::Frightened {
            let random_idx = rng.random_range(0..possible_dirs.len());
            return Some(possible_dirs[random_idx]);
        }

        // 返回基地时用 A* 寻路
        if self.mode == GhostMode::ReturnToBase {
            // TODO: 暂时采用最短距离
            return possible_dirs
                .iter()
                .min_by_key(|&dir| {
                    let new_pos = self.tile_pos + dir;
                    (new_pos - target_pos).length_squared()
                })
                .copied();
        }

        // 正常模式下选择最接近目标的方向
        possible_dirs
            .iter()
            .min_by_key(|&dir| {
                let new_pos = self.tile_pos + dir;
                (new_pos - target_pos).length_squared()
            })
            .copied()
    }

    /// 更新状态计时器
    ///
    /// ## Params
    /// - delta_time: 时间增量，表示上一帧到当前帧经过的时间（单位:秒）
    pub fn update_timers(&mut self, delta_secs: f32) {
        // 更新离开基地的延迟
        if self.in_house {
            self.house_delay -= delta_secs;
            self.try_leave_house();
        }

        match self.mode {
            GhostMode::Frightened => {
                self.frightened_time -= delta_secs;
                // 恐惧状态结束，追赶玩家、恢复速度
                if self.frightened_time <= 0.0 {
                    self.mode = GhostMode::Chase;
                    self.movement.speed = 4.0;
                }
            }
            GhostMode::Scatter | GhostMode::Chase => {
                // 每20s在分散和追逐之间切换
                self.scatter_timer += delta_secs;
                if self.scatter_timer >= 20.0 {
                    self.mode = if self.mode == GhostMode::Scatter {
                        GhostMode::Chase
                    } else {
                        GhostMode::Scatter
                    };
                    self.scatter_timer = 0.0;
                }
            }
            GhostMode::ReturnToBase => {
                // 回到基地后恢复正常
                if self.tile_pos == self.spawn_pos {
                    self.mode = GhostMode::Chase;
                    self.movement.speed = 4.0;
                }
            }
        }
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
            // self.movement.stop_moving();
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
