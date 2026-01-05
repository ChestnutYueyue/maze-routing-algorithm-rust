//! BFS (广度优先搜索) 算法模块

use std::collections::{VecDeque, HashMap};
use crate::map::Map;
use crate::point::{Point, Direction, Color};

/// 方向偏移量: (dx, dy, direction)
const DIRECTIONS: [(i32, i32, Direction); 4] = [
    (0, 1, Direction::Down),
    (1, 0, Direction::Right),
    (0, -1, Direction::Up),
    (-1, 0, Direction::Left),
];

/// BFS 搜索结果
#[derive(Debug)]
pub struct BfsResult {
    pub found: bool,
    pub steps: Vec<Point>,
    pub path_length: i32,
    pub path: Vec<Point>,
}

/// BFS 搜索器
pub struct BFS {
    map: Map,
    sx: i32,
    sy: i32,
    ex: i32,
    ey: i32,
}

impl BFS {
    /// 创建新的 BFS 搜索器
    pub fn new(map: Map, sx: i32, sy: i32, ex: i32, ey: i32) -> Self {
        Self { map, sx, sy, ex, ey }
    }
    
    /// 执行搜索（返回所有步骤，用于可视化）
    pub fn search(&mut self) -> BfsResult {
        let mut steps = Vec::new();
        let mut queue = VecDeque::new();
        let mut pre: HashMap<(i32, i32), Point> = HashMap::new();
        
        let start = Point::with_color(self.sx, self.sy, Color::YELLOW);
        let end = Point::new(self.ex, self.ey);
        
        queue.push_back(start.clone());
        self.map.set_cell(self.sx, self.sy, 4);
        steps.push(start.clone());
        
        let mut found = false;
        let mut final_step = 0;
        
        while let Some(current) = queue.pop_front() {
            steps.push(current.clone());
            
            if current == end {
                let mut end_point = current.clone();
                end_point.color = Color::YELLOW;
                steps.push(end_point.clone());
                
                // 回溯路径
                let mut path_point = end_point;
                while path_point != start {
                    self.map.set_cell(path_point.x, path_point.y, 3);
                    if let Some(prev) = pre.get(&(path_point.x, path_point.y)) {
                        let mut prev_display = prev.clone();
                        prev_display.color = Color::LIGHT_RED;
                        prev_display.direction = prev.direction.reverse();
                        steps.push(prev_display.clone());
                        path_point = prev.clone();
                    } else {
                        break;
                    }
                }
                
                let mut final_start = start.clone();
                final_start.color = Color::YELLOW;
                steps.push(final_start);
                
                found = true;
                final_step = current.step;
                break;
            }
            
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
                    queue.push_back(next);
                    pre.insert((nx, ny), Point::full(
                        current.x, current.y,
                        current.step,
                        0,
                        *dir,
                        current.color
                    ));
                    self.map.set_cell(nx, ny, 2);
                }
            }
        }
        
        // 构建最终路径
        let path = self.reconstruct_path(&pre, &start, &end);
        
        BfsResult {
            found,
            steps,
            path_length: final_step,
            path,
        }
    }
    
    /// 重建路径
    fn reconstruct_path(&self, pre: &HashMap<(i32, i32), Point>, start: &Point, end: &Point) -> Vec<Point> {
        let mut path = Vec::new();
        let mut current = end.clone();
        
        while current != *start {
            path.push(current.clone());
            if let Some(prev) = pre.get(&(current.x, current.y)) {
                current = prev.clone();
            } else {
                break;
            }
        }
        path.push(start.clone());
        path.reverse();
        path
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

/// 执行 BFS 搜索的便捷函数
pub fn bfs_search(map: &mut Map, sx: i32, sy: i32, ex: i32, ey: i32) -> BfsResult {
    let mut bfs = BFS::new(map.clone(), sx, sy, ex, ey);
    let result = bfs.search();
    *map = bfs.map;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bfs_basic() {
        let mut map = Map::with_size(10, 10, 100, 100);
        let (sx, sy) = map.start();
        let (ex, ey) = map.end();
        let result = bfs_search(&mut map, sx, sy, ex, ey);
        println!("BFS found: {}, path length: {}", result.found, result.path_length);
    }
}
