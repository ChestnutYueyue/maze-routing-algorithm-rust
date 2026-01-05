//! 点结构体模块 - 表示迷宫中的坐标点

/// 方向枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// 获取方向的显示字符
    pub fn to_char(&self) -> char {
        match self {
            Direction::None => ' ',
            Direction::Up => '↑',
            Direction::Down => '↓',
            Direction::Left => '←',
            Direction::Right => '→',
        }
    }

    /// 获取反方向
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::None => Direction::None,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

/// 颜色结构体 (RGB)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// 转换为 u32 格式 (用于 minifb)
    pub fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    // 预定义颜色
    pub const WHITE: Color = Color::new(255, 255, 255);
    pub const BLACK: Color = Color::new(0, 10, 0);
    pub const YELLOW: Color = Color::new(255, 255, 0);
    pub const SKY_BLUE: Color = Color::new(135, 206, 235);
    pub const LIGHT_RED: Color = Color::new(255, 106, 106);
    pub const AQUAMARINE: Color = Color::new(127, 255, 212);
}

/// 点结构体
#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,           // x坐标
    pub y: i32,           // y坐标
    pub step: i32,        // g值：实际路径代价
    pub h_cost: i32,      // h值：启发式代价（用于A*）
    pub direction: Direction, // 方向
    pub color: Color,     // 颜色
}

impl Point {
    /// 创建新点
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            step: 0,
            h_cost: 0,
            direction: Direction::None,
            color: Color::WHITE,
        }
    }

    /// 创建带步数的点
    pub fn with_step(x: i32, y: i32, step: i32) -> Self {
        Self {
            x,
            y,
            step,
            h_cost: 0,
            direction: Direction::None,
            color: Color::WHITE,
        }
    }

    /// 创建带颜色的点
    pub fn with_color(x: i32, y: i32, color: Color) -> Self {
        Self {
            x,
            y,
            step: 0,
            h_cost: 0,
            direction: Direction::None,
            color,
        }
    }

    /// 创建带方向的点
    pub fn with_direction(x: i32, y: i32, direction: Direction) -> Self {
        Self {
            x,
            y,
            step: 0,
            h_cost: 0,
            direction,
            color: Color::WHITE,
        }
    }

    /// 创建完整的点
    pub fn full(x: i32, y: i32, step: i32, h_cost: i32, direction: Direction, color: Color) -> Self {
        Self {
            x,
            y,
            step,
            h_cost,
            direction,
            color,
        }
    }

    /// 获取 f 值 (用于 A* 算法)
    pub fn f_cost(&self) -> i32 {
        self.step + self.h_cost
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl std::hash::Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
