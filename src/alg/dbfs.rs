//! DBFS (双向广度优先搜索) 算法模块

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

/// DBFS 搜索结果
#[derive(Debug)]
pub struct DbfsResult {
    pub found: bool,
    pub steps: Vec<Point>,
    pub path_length: i32,
    pub path: Vec<Point>,
}

/// DBFS 搜索器
pub struct DBFS {
    map: Map,
    sx: i32,
    sy: i32,
    ex: i32,
    ey: i32,
}

impl DBFS {
    /// 创建新的 DBFS 搜索器
    pub fn new(map: Map, sx: i32, sy: i32, ex: i32, ey: i32) -> Self {
        Self { map, sx, sy, ex, ey }
    }
    
    /// 执行搜索（返回所有步骤，用于可视化）
    pub fn search(&mut self) -> DbfsResult {
        let mut steps = Vec::new();
        let mut start_queue = VecDeque::new();
        let mut end_queue = VecDeque::new();
        let mut start_pre: HashMap<(i32, i32), Point> = HashMap::new();
        let mut end_pre: HashMap<(i32, i32), Point> = HashMap::new();
        
        let start = Point::with_color(self.sx, self.sy, Color::YELLOW);
        let end = Point::with_color(self.ex, self.ey, Color::YELLOW);
        
        start_queue.push_back(start.clone());
        end_queue.push_back(end.clone());
        self.map.set_cell(self.sx, self.sy, 4);
        self.map.set_cell(self.ex, self.ey, 4);
        steps.push(start.clone());
        
        let mut found = false;
        let mut meeting_point: Option<(Point, Point)> = None;
        
        while !start_queue.is_empty() && !end_queue.is_empty() {
            // 选择较小的队列进行扩展
            let expand_start = start_queue.len() <= end_queue.len();
            
            let (x, y, step, color) = if expand_start {
                let current = start_queue.pop_front().unwrap();
                (current.x, current.y, current.step, current.color)
            } else {
                let current = end_queue.pop_front().unwrap();
                (current.x, current.y, current.step, current.color)
            };
            
            steps.push(Point::full(x, y, step, 0, Direction::None, color));
            
            for (dx, dy, dir) in DIRECTIONS.iter() {
                let nx = x + dx;
                let ny = y + dy;
                
                if !self.map.in_bounds(nx, ny) {
                    continue;
                }
                
                let cell = self.map.get_cell(nx, ny);
                
                if cell == 1 {
                    continue; // 墙
                }
                
                if cell == 0 {
                    // 未访问的格子
                    if expand_start {
                        self.map.set_cell(nx, ny, 2);
                        start_queue.push_back(Point::full(
                            nx, ny,
                            step + 1,
                            0,
                            *dir,
                            Color::SKY_BLUE
                        ));
                        start_pre.insert((nx, ny), Point::with_direction(x, y, *dir));
                    } else {
                        self.map.set_cell(nx, ny, 3);
                        end_queue.push_back(Point::full(
                            nx, ny,
                            step + 1,
                            0,
                            *dir,
                            Color::LIGHT_RED
                        ));
                        end_pre.insert((nx, ny), Point::with_direction(x, y, *dir));
                    }
                } else if (cell == 2 && !expand_start) || (cell == 3 && expand_start) {
                    // 两个搜索相遇
                    let (start_point, end_point) = if expand_start {
                        (Point::with_direction(x, y, *dir), Point::with_direction(nx, ny, dir.reverse()))
                    } else {
                        (Point::with_direction(nx, ny, dir.reverse()), Point::with_direction(x, y, *dir))
                    };
                    meeting_point = Some((start_point, end_point));
                    found = true;
                    break;
                }
            }
            
            if found {
                break;
            }
        }
        
        // 构建最终路径
        let path = if let Some((start_meet, end_meet)) = meeting_point {
            self.reconstruct_path(&start_pre, &end_pre, &start, &end, &start_meet, &end_meet, &mut steps)
        } else {
            Vec::new()
        };
        
        let path_length = path.len() as i32 - 1;
        
        DbfsResult {
            found,
            steps,
            path_length: if path_length < 0 { 0 } else { path_length },
            path,
        }
    }
    
    /// 重建路径
    fn reconstruct_path(
        &mut self,
        start_pre: &HashMap<(i32, i32), Point>,
        end_pre: &HashMap<(i32, i32), Point>,
        start: &Point,
        end: &Point,
        start_meet: &Point,
        end_meet: &Point,
        steps: &mut Vec<Point>
    ) -> Vec<Point> {
        let mut path = Vec::new();
        
        // 从起点到相遇点的路径
        let mut start_path = Vec::new();
        let mut current = start_meet.clone();
        while current != *start {
            start_path.push(current.clone());
            if let Some(prev) = start_pre.get(&(current.x, current.y)) {
                current = prev.clone();
            } else {
                break;
            }
        }
        start_path.push(start.clone());
        start_path.reverse();
        
        // 从相遇点到终点的路径
        let mut end_path = Vec::new();
        current = end_meet.clone();
        while current != *end {
            let mut display = current.clone();
            display.direction = display.direction.reverse();
            end_path.push(display);
            if let Some(prev) = end_pre.get(&(current.x, current.y)) {
                current = prev.clone();
            } else {
                break;
            }
        }
        end_path.push(end.clone());
        
        // 合并路径
        path.extend(start_path);
        path.extend(end_path);
        
        // 可视化最终路径
        let mut step_count = 0;
        for p in &path {
            self.map.set_cell(p.x, p.y, 5);
            step_count += 1;
            steps.push(Point::full(
                p.x, p.y,
                step_count,
                0,
                p.direction,
                Color::AQUAMARINE
            ));
        }
        
        // 标记终点
        steps.push(Point::full(
            end.x, end.y,
            step_count,
            0,
            Direction::None,
            Color::YELLOW
        ));
        
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

/// 执行 DBFS 搜索的便捷函数
pub fn dbfs_search(map: &mut Map, sx: i32, sy: i32, ex: i32, ey: i32) -> DbfsResult {
    let mut dbfs = DBFS::new(map.clone(), sx, sy, ex, ey);
    let result = dbfs.search();
    *map = dbfs.map;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dbfs_basic() {
        let mut map = Map::with_size(10, 10, 100, 100);
        let (sx, sy) = map.start();
        let (ex, ey) = map.end();
        let result = dbfs_search(&mut map, sx, sy, ex, ey);
        println!("DBFS found: {}, path length: {}", result.found, result.path_length);
    }
}
