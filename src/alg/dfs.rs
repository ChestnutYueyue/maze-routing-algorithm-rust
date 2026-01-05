//! DFS (深度优先搜索) 算法模块

use crate::map::Map;
use crate::point::{Point, Direction, Color};

/// 方向偏移量: (dx, dy, direction)
const DIRECTIONS: [(i32, i32, Direction); 4] = [
    (0, 1, Direction::Down),
    (1, 0, Direction::Right),
    (0, -1, Direction::Up),
    (-1, 0, Direction::Left),
];

/// DFS 搜索结果
#[derive(Debug)]
pub struct DfsResult {
    pub found: bool,
    pub steps: Vec<Point>,
    pub path_length: i32,
}

/// DFS 搜索器
pub struct DFS {
    map: Map,
    sx: i32,
    sy: i32,
    ex: i32,
    ey: i32,
}

impl DFS {
    /// 创建新的 DFS 搜索器
    pub fn new(map: Map, sx: i32, sy: i32, ex: i32, ey: i32) -> Self {
        Self { map, sx, sy, ex, ey }
    }
    
    /// 执行搜索（返回所有步骤，用于可视化）
    pub fn search(&mut self) -> DfsResult {
        let mut steps = Vec::new();
        let mut stack = Vec::new();
        
        let start = Point::with_color(self.sx, self.sy, Color::YELLOW);
        let end = Point::new(self.ex, self.ey);
        
        stack.push(start.clone());
        self.map.set_cell(self.sx, self.sy, 4);
        steps.push(start.clone());
        
        let mut found = false;
        let mut final_step = 0;
        
        while let Some(current) = stack.last().cloned() {
            steps.push(current.clone());
            
            if current == end {
                let mut end_point = current.clone();
                end_point.color = Color::YELLOW;
                end_point.direction = Direction::None;
                steps.push(end_point);
                found = true;
                final_step = current.step;
                break;
            }
            
            // 查找下一个可访问的邻居
            let mut found_next = false;
            for (dx, dy, dir) in DIRECTIONS.iter() {
                let nx = current.x + dx;
                let ny = current.y + dy;
                
                if self.map.in_bounds(nx, ny) && self.map.get_cell(nx, ny) == 0 {
                    let next = Point::full(
                        nx, ny, 
                        current.step + 1, 
                        0,
                        *dir, 
                        Color::SKY_BLUE
                    );
                    stack.push(next);
                    self.map.set_cell(nx, ny, 2);
                    found_next = true;
                    break;
                }
            }
            
            if !found_next {
                // 回溯
                stack.pop();
                let mut backtrack = current.clone();
                backtrack.step = current.step - 1;
                backtrack.direction = Direction::None;
                backtrack.color = Color::LIGHT_RED;
                self.map.set_cell(current.x, current.y, 3);
                steps.push(backtrack);
            }
        }
        
        DfsResult {
            found,
            steps,
            path_length: final_step,
        }
    }
    
    /// 获取地图引用
    pub fn map(&self) -> &Map {
        &self.map
    }
    
    /// 获取可变地图引用
    pub fn map_mut(&mut self) -> &mut Map {
        &mut self.map
    }
}

/// 执行 DFS 搜索的便捷函数
pub fn dfs_search(map: &mut Map, sx: i32, sy: i32, ex: i32, ey: i32) -> DfsResult {
    let mut dfs = DFS::new(map.clone(), sx, sy, ex, ey);
    let result = dfs.search();
    *map = dfs.map;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dfs_basic() {
        let mut map = Map::with_size(10, 10, 100, 100);
        let (sx, sy) = map.start();
        let (ex, ey) = map.end();
        let result = dfs_search(&mut map, sx, sy, ex, ey);
        // DFS 不保证找到路径（如果迷宫没有通路）
        println!("DFS found: {}, steps: {}", result.found, result.steps.len());
    }
}
