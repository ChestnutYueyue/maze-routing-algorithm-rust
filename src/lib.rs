//! 迷宫寻路算法库
//! 
//! 本库实现了多种迷宫寻路算法：
//! - DFS (深度优先搜索)
//! - BFS (广度优先搜索)
//! - DBFS (双向广度优先搜索)
//! - A* (A-Star 启发式搜索)

pub mod core;
pub mod alg;

pub use core::point::{Point, Direction, Color};
pub use core::map::Map;
pub use core::{map, point};
pub use alg::{Algorithm, DFS, DfsResult, dfs_search};
pub use alg::{BFS, BfsResult, bfs_search};
pub use alg::{DBFS, DbfsResult, dbfs_search};
pub use alg::{AStar, AStarResult, astar_search};
pub use alg::run_algorithm;
