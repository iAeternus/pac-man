use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    BLINKY_COLOR, CLYDE_COLOR, GHOST_EYE_PUPIL_COLOR, GHOST_EYE_RADIUS, GHOST_EYE_WHITE_COLOR,
    GHOST_FRIGHTENED_COLOR, GHOST_FRIGHTENED_FLASH_COLOR, GHOST_LEFT_EYE_POSITION,
    GHOST_PUPIL_RADIUS, GHOST_RADIUS, GHOST_RIGHT_EYE_POSITION, GHOST_STROKE_COLOR, Ghost,
    GhostMode, GhostType, HALF, INKY_COLOR, PINKY_COLOR, TILE_SIZE, Z_GHOST_BODY,
    Z_GHOST_EYE_PUPIL, Z_GHOST_EYE_WHITE,
};

/// 幽灵UI标记组件
#[derive(Component)]
pub struct GhostUI;

/// 幽灵渲染组件
#[derive(Component, Clone)]
pub struct GhostRenderer {
    pub ghost_type: GhostType,
    pub is_frightened: bool,
    pub frightened_timer: f32,
}

impl GhostRenderer {
    pub fn new(ghost_type: GhostType) -> Self {
        Self {
            ghost_type,
            is_frightened: false,
            frightened_timer: 0.0,
        }
    }

    /// 获取幽灵颜色
    pub fn get_color(&self) -> Color {
        if self.is_frightened {
            // 恐惧状态：蓝白闪烁
            if self.frightened_timer > 2.0 || (self.frightened_timer * 10.0).floor() as i32 % 2 == 0
            {
                GHOST_FRIGHTENED_COLOR
            } else {
                GHOST_FRIGHTENED_FLASH_COLOR
            }
        } else {
            // 正常状态：根据幽灵类型
            match self.ghost_type {
                GhostType::Blinky => BLINKY_COLOR,
                GhostType::Pinky => PINKY_COLOR,
                GhostType::Inky => INKY_COLOR,
                GhostType::Clyde => CLYDE_COLOR,
            }
        }
    }

    /// 获取幽灵的z轴值
    pub fn get_z_index(&self) -> f32 {
        match self.ghost_type {
            GhostType::Blinky => Z_GHOST_BODY + 0.3,
            GhostType::Pinky => Z_GHOST_BODY + 0.2,
            GhostType::Inky => Z_GHOST_BODY + 0.1,
            GhostType::Clyde => Z_GHOST_BODY,
        }
    }
}

/// 创建幽灵UI系统
pub fn spawn_ghost_ui(
    mut commands: Commands,
    map_data: Res<crate::MapData>,
    query: Query<(Entity, &Ghost), Without<GhostUI>>,
) {
    // 计算地图偏移量（与世界坐标系一致）
    let offset_x = -((map_data.width as f32) * TILE_SIZE) / 2.0;
    let offset_y = ((map_data.height as f32) * TILE_SIZE) / 2.0;

    for (entity, ghost) in query.iter() {
        let ghost_renderer = GhostRenderer::new(ghost.ghost_type);
        let z_index = ghost_renderer.get_z_index();

        // 计算幽灵位置
        let px = offset_x + ghost.tile_pos.x as f32 * TILE_SIZE;
        let py = offset_y - ghost.tile_pos.y as f32 * TILE_SIZE;

        commands
            .entity(entity)
            .insert(GhostUI)
            .insert(ghost_renderer.clone())
            .insert(Visibility::default())
            .insert(Transform::from_xyz(px + HALF, py - HALF, z_index))
            .with_children(|parent| {
                draw_ghost(parent, ghost, &ghost_renderer);
            });
    }
}

/// 更新幽灵UI系统
pub fn update_ghost_ui(
    mut commands: Commands,
    map_data: Res<crate::MapData>,
    mut query: Query<(Entity, &Ghost, &mut GhostRenderer, &mut Transform), With<GhostUI>>,
) {
    // 计算地图偏移量
    let offset_x = -((map_data.width as f32) * TILE_SIZE) / 2.0;
    let offset_y = ((map_data.height as f32) * TILE_SIZE) / 2.0;

    for (entity, ghost, mut renderer, mut transform) in query.iter_mut() {
        // 更新位置
        let px = offset_x + ghost.tile_pos.x as f32 * TILE_SIZE;
        let py = offset_y - ghost.tile_pos.y as f32 * TILE_SIZE;
        transform.translation.x = px + HALF;
        transform.translation.y = py - HALF;

        // 更新z轴
        transform.translation.z = renderer.get_z_index();

        // 更新渲染器状态
        renderer.is_frightened = ghost.mode == GhostMode::Frightened;
        renderer.frightened_timer = ghost.frightened_time;

        // 清理旧的形状组件
        commands.entity(entity).despawn_children();

        // 重新创建幽灵图形
        commands.entity(entity).with_children(|parent| {
            draw_ghost(parent, ghost, &renderer);
        });
    }
}

/// 绘制幽灵形状
fn draw_ghost(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    ghost: &Ghost,
    renderer: &GhostRenderer,
) {
    let color = renderer.get_color();

    // 只在非返回基地状态下绘制身体
    if ghost.mode != GhostMode::ReturnToBase {
        // 幽灵主体: 圆形身体
        let body_shape = shapes::Circle {
            radius: GHOST_RADIUS,
            center: Vec2::ZERO,
        };

        parent.spawn((
            ShapeBuilder::with(&body_shape)
                .fill(Fill::color(color))
                .stroke(Stroke::new(GHOST_STROKE_COLOR, 1.0))
                .build(),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }

    // 绘制眼睛（总是绘制）
    draw_ghost_eyes(parent, ghost, renderer);
}

/// 绘制幽灵眼睛
fn draw_ghost_eyes(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    ghost: &Ghost,
    renderer: &GhostRenderer,
) {
    let pupil_offset = if renderer.is_frightened {
        Vec2::ZERO // 恐惧状态下瞳孔居中
    } else {
        // 正常状态下瞳孔看向移动方向
        Vec2::new(
            ghost.movement.direction.x as f32 * 1.5,
            -ghost.movement.direction.y as f32 * 1.5,
        )
    };

    // 左眼白
    parent.spawn((
        ShapeBuilder::with(&shapes::Circle {
            radius: GHOST_EYE_RADIUS,
            center: Vec2::new(GHOST_LEFT_EYE_POSITION.0, GHOST_LEFT_EYE_POSITION.1),
        })
        .fill(Fill::color(GHOST_EYE_WHITE_COLOR))
        .build(),
        Transform::from_xyz(0.0, 0.0, Z_GHOST_EYE_WHITE),
    ));

    // 右眼白
    parent.spawn((
        ShapeBuilder::with(&shapes::Circle {
            radius: GHOST_EYE_RADIUS,
            center: Vec2::new(GHOST_RIGHT_EYE_POSITION.0, GHOST_RIGHT_EYE_POSITION.1),
        })
        .fill(Fill::color(GHOST_EYE_WHITE_COLOR))
        .build(),
        Transform::from_xyz(0.0, 0.0, Z_GHOST_EYE_WHITE),
    ));

    // 左瞳孔
    parent.spawn((
        ShapeBuilder::with(&shapes::Circle {
            radius: GHOST_PUPIL_RADIUS,
            center: Vec2::new(GHOST_LEFT_EYE_POSITION.0, GHOST_LEFT_EYE_POSITION.1) + pupil_offset,
        })
        .fill(Fill::color(if renderer.is_frightened {
            GHOST_EYE_WHITE_COLOR
        } else {
            GHOST_EYE_PUPIL_COLOR
        }))
        .build(),
        Transform::from_xyz(0.0, 0.0, Z_GHOST_EYE_PUPIL),
    ));

    // 右瞳孔
    parent.spawn((
        ShapeBuilder::with(&shapes::Circle {
            radius: GHOST_PUPIL_RADIUS,
            center: Vec2::new(GHOST_RIGHT_EYE_POSITION.0, GHOST_RIGHT_EYE_POSITION.1)
                + pupil_offset,
        })
        .fill(Fill::color(if renderer.is_frightened {
            GHOST_EYE_WHITE_COLOR
        } else {
            GHOST_EYE_PUPIL_COLOR
        }))
        .build(),
        Transform::from_xyz(0.0, 0.0, Z_GHOST_EYE_PUPIL),
    ));
}
