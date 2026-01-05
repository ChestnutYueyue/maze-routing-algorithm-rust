pub mod dfs;
pub mod bfs;
pub mod dbfs;
pub mod astar;

pub use dfs::{DFS, DfsResult, dfs_search};
pub use bfs::{BFS, BfsResult, bfs_search};
pub use dbfs::{DBFS, DbfsResult, dbfs_search};
pub use astar::{AStar, AStarResult, astar_search};

use crate::{Map, Point};

/// 搜索算法类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    DFS,
    BFS,
    DBFS,
    AStar,
}

impl Algorithm {
    /// 获取算法名称
    pub fn name(&self) -> &'static str {
        match self {
            Algorithm::DFS => "DFS (深度优先搜索)",
            Algorithm::BFS => "BFS (广度优先搜索)",
            Algorithm::DBFS => "DBFS (双向广度优先搜索)",
            Algorithm::AStar => "A* (A-Star 启发式搜索)",
        }
    }
    
    /// 获取算法简称
    pub fn short_name(&self) -> &'static str {
        match self {
            Algorithm::DFS => "DFS",
            Algorithm::BFS => "BFS",
            Algorithm::DBFS => "DBFS",
            Algorithm::AStar => "A*",
        }
    }
}

/// 运行指定算法并返回步骤、是否找到、路径长度
pub fn run_algorithm(map: &mut Map, alg: Algorithm) -> (Vec<Point>, bool, i32) {
    let (sx, sy) = map.start();
    let (ex, ey) = map.end();
    match alg {
        Algorithm::DFS => {
            let r = dfs_search(map, sx, sy, ex, ey);
            (r.steps, r.found, r.path_length)
        }
        Algorithm::BFS => {
            let r = bfs_search(map, sx, sy, ex, ey);
            (r.steps, r.found, r.path_length)
        }
        Algorithm::DBFS => {
            let r = dbfs_search(map, sx, sy, ex, ey);
            (r.steps, r.found, r.path_length)
        }
        Algorithm::AStar => {
            let r = astar_search(map, sx, sy, ex, ey);
            (r.steps, r.found, r.path_length)
        }
    }
}
