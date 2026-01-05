//! A* (A-Star) 算法模块

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use crate::map::Map;
use crate::point::{Point, Direction, Color};

/// 方向偏移量: (dx, dy, direction)
const DIRECTIONS: [(i32, i32, Direction); 4] = [
    (0, 1, Direction::Down),
    (1, 0, Direction::Right),
    (0, -1, Direction::Up),
    (-1, 0, Direction::Left),
];

/// 用于优先队列的节点包装
#[derive(Debug, Clone)]
struct AStarNode {
    point: Point,
    f_cost: i32,
}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.f_cost == other.f_cost
    }
}

impl Eq for AStarNode {}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // 反转顺序，使 BinaryHeap 成为最小堆
        other.f_cost.cmp(&self.f_cost)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A* 搜索结果
#[derive(Debug)]
pub struct AStarResult {
    pub found: bool,
    pub steps: Vec<Point>,
    pub path_length: i32,
    pub path: Vec<Point>,
}

/// A* 搜索器
pub struct AStar {
    map: Map,
    sx: i32,
    sy: i32,
    ex: i32,
    ey: i32,
}

impl AStar {
    /// 创建新的 A* 搜索器
    pub fn new(map: Map, sx: i32, sy: i32, ex: i32, ey: i32) -> Self {
        Self { map, sx, sy, ex, ey }
    }
    
    /// 计算曼哈顿距离（启发式函数）
    fn heuristic(&self, x: i32, y: i32) -> i32 {
        (self.ex - x).abs() + (self.ey - y).abs()
    }
    
    /// 执行搜索（返回所有步骤，用于可视化）
    pub fn search(&mut self) -> AStarResult {
        let mut steps = Vec::new();
        let mut open_set = BinaryHeap::new();
        let mut pre: HashMap<(i32, i32), Point> = HashMap::new();
        
        let mut start = Point::with_color(self.sx, self.sy, Color::YELLOW);
        start.step = 0;
        start.h_cost = self.heuristic(self.sx, self.sy);
        
        let end = Point::new(self.ex, self.ey);
        
        open_set.push(AStarNode {
            point: start.clone(),
            f_cost: start.f_cost(),
        });
        self.map.set_cell(self.sx, self.sy, 4);
        steps.push(start.clone());
        
        let mut found = false;
        let mut final_step = 0;
        
        while let Some(AStarNode { point: current, .. }) = open_set.pop() {
            steps.push(current.clone());
            
            if current == end {
                let mut end_point = current.clone();
                end_point.color = Color::YELLOW;
                steps.push(end_point.clone());
                
                // 回溯路径
                let mut path_point = end.clone();
                while path_point != start {
                    self.map.set_cell(path_point.x, path_point.y, 3);
                    if let Some(prev) = pre.get(&(path_point.x, path_point.y)) {
                        let mut prev_display = prev.clone();
                        prev_display.color = Color::LIGHT_RED;
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
                
                if !self.map.in_bounds(nx, ny) || self.map.get_cell(nx, ny) != 0 {
                    continue;
                }
                
                let mut neighbor = Point::new(nx, ny);
                neighbor.step = current.step + 1;
                neighbor.h_cost = self.heuristic(nx, ny);
                neighbor.direction = *dir;
                neighbor.color = Color::SKY_BLUE;
                
                open_set.push(AStarNode {
                    f_cost: neighbor.f_cost(),
                    point: neighbor,
                });
                
                pre.insert((nx, ny), current.clone());
                self.map.set_cell(nx, ny, 2);
            }
        }
        
        // 构建最终路径
        let path = self.reconstruct_path(&pre, &start, &end);
        
        AStarResult {
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

/// 执行 A* 搜索的便捷函数
pub fn astar_search(map: &mut Map, sx: i32, sy: i32, ex: i32, ey: i32) -> AStarResult {
    let mut astar = AStar::new(map.clone(), sx, sy, ex, ey);
    let result = astar.search();
    *map = astar.map;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_astar_basic() {
        let mut map = Map::with_size(10, 10, 100, 100);
        let (sx, sy) = map.start();
        let (ex, ey) = map.end();
        let result = astar_search(&mut map, sx, sy, ex, ey);
        println!("A* found: {}, path length: {}", result.found, result.path_length);
    }
}
