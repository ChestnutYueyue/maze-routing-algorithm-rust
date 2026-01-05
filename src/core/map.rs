//! 地图模块 - 迷宫地图的生成与管理

use rand::seq::SliceRandom;
use crate::point::Color;

/// 单元格状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Wall = 1,       // 墙
    Path = 0,       // 通道
    Visited = 2,    // 已访问（搜索中）
    Backtrack = 3,  // 回溯（已探索）
    Start = 4,      // 起点/终点
    Solution = 5,   // 最终路径
}

/// 迷宫地图结构体
#[derive(Debug, Clone)]
pub struct Map {
    pub sx: i32,           // 起点 x
    pub sy: i32,           // 起点 y
    pub m: i32,            // 地图宽度
    pub n: i32,            // 地图高度
    pub width: i32,        // 像素宽度
    pub height: i32,       // 像素高度
    pub grid: Vec<Vec<i32>>, // 地图数据
}

impl Map {
    /// 创建新地图
    pub fn new() -> Self {
        Self::with_size(56, 56, 560, 560)
    }

    /// 创建指定大小的地图
    pub fn with_size(m: i32, n: i32, width: i32, height: i32) -> Self {
        let mut map = Self {
            sx: 1,
            sy: 1,
            m,
            n,
            width,
            height,
            grid: vec![vec![1; (m + 1) as usize]; (n + 1) as usize],
        };
        map.generate();
        map
    }

    /// Kruskal + 并查集生成完美迷宫（奇数坐标为通道，偶数坐标为墙）
    fn generate(&mut self) {
        let mut rng = rand::thread_rng();

        // 逻辑单元格数量（仅使用奇数坐标作为可通行单元）
        let cells_x = (self.m / 2) as usize;
        let cells_y = (self.n / 2) as usize;

        // 收集左右/上下相邻单元格之间的墙
        let mut edges: Vec<((i32, i32), (i32, i32))> = Vec::new();
        let mut y = 1;
        while y < self.n {
            let mut x = 1;
            while x < self.m {
                if x + 2 < self.m {
                    edges.push(((x, y), (x + 2, y)));
                }
                if y + 2 < self.n {
                    edges.push(((x, y), (x, y + 2)));
                }
                x += 2;
            }
            y += 2;
        }
        edges.shuffle(&mut rng);

        let mut uf = UnionFind::new(cells_x * cells_y);

        // 初始化全部为墙，保证起点/终点可通行
        self.grid[self.sy as usize][self.sx as usize] = 0;
        self.grid[(self.n - 1) as usize][(self.m - 1) as usize] = 0;

        // Kruskal 逐墙打通
        for ((x1, y1), (x2, y2)) in edges {
            let i1 = Self::cell_idx(x1, y1, cells_x);
            let i2 = Self::cell_idx(x2, y2, cells_x);
            if uf.find(i1) != uf.find(i2) {
                uf.union(i1, i2);

                // 打通两个单元格及其中间的墙
                let wx = (x1 + x2) / 2;
                let wy = (y1 + y2) / 2;
                self.grid[y1 as usize][x1 as usize] = 0;
                self.grid[y2 as usize][x2 as usize] = 0;
                self.grid[wy as usize][wx as usize] = 0;
            }
        }
    }

    #[inline]
    fn cell_idx(x: i32, y: i32, cells_x: usize) -> usize {
        (y as usize / 2) * cells_x + (x as usize / 2)
    }
    
    /// 检查坐标是否在地图范围内
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 1 && x <= self.m && y >= 1 && y <= self.n
    }
    
    /// 检查坐标是否可通行
    pub fn is_passable(&self, x: i32, y: i32) -> bool {
        self.in_bounds(x, y) && self.grid[y as usize][x as usize] == 0
    }
    
    /// 获取单元格状态
    pub fn get_cell(&self, x: i32, y: i32) -> i32 {
        if self.in_bounds(x, y) {
            self.grid[y as usize][x as usize]
        } else {
            1 // 越界视为墙
        }
    }
    
    /// 设置单元格状态
    pub fn set_cell(&mut self, x: i32, y: i32, value: i32) {
        if self.in_bounds(x, y) {
            self.grid[y as usize][x as usize] = value;
        }
    }
    
    /// 获取起点坐标
    pub fn start(&self) -> (i32, i32) {
        (self.sx, self.sy)
    }
    
    /// 获取终点坐标
    pub fn end(&self) -> (i32, i32) {
        (self.m - 1, self.n - 1)
    }
    
    /// 渲染地图到像素缓冲区
    pub fn render(&self, buffer: &mut Vec<u32>, buf_width: usize, buf_height: usize) {
        let cell_w = buf_width / (self.m + 1) as usize;
        let cell_h = buf_height / (self.n + 1) as usize;
        
        for ny in 0..=self.n as usize {
            for nx in 0..=self.m as usize {
                let color = match self.grid[ny][nx] {
                    0 => Color::WHITE,
                    1 => Color::BLACK,
                    2 => Color::SKY_BLUE,
                    3 => Color::LIGHT_RED,
                    4 => Color::YELLOW,
                    5 => Color::AQUAMARINE,
                    _ => Color::WHITE,
                };
                
                let color_u32 = color.to_u32();
                
                for py in (ny * cell_h)..((ny + 1) * cell_h).min(buf_height) {
                    for px in (nx * cell_w)..((nx + 1) * cell_w).min(buf_width) {
                        buffer[py * buf_width + px] = color_u32;
                    }
                }
            }
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

// 简单的并查集实现（路径压缩 + 按大小合并）
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut pa = self.find(a);
        let mut pb = self.find(b);
        if pa == pb {
            return;
        }
        if self.size[pa] < self.size[pb] {
            std::mem::swap(&mut pa, &mut pb);
        }
        self.parent[pb] = pa;
        self.size[pa] += self.size[pb];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_map_creation() {
        let map = Map::new();
        assert_eq!(map.m, 56);
        assert_eq!(map.n, 56);
        assert_eq!(map.sx, 1);
        assert_eq!(map.sy, 1);
    }
    
    #[test]
    fn test_in_bounds() {
        let map = Map::with_size(10, 10, 100, 100);
        assert!(map.in_bounds(1, 1));
        assert!(map.in_bounds(10, 10));
        assert!(!map.in_bounds(0, 0));
        assert!(!map.in_bounds(11, 11));
    }
}
