//! 迷宫寻路算法 - GUI 演示版本 (使用 egui)

mod config;
mod render;
mod ui;
mod input;

use eframe::egui;
use maze_routing_algorithm::*;
use std::time::Instant;
use config::{MAP_SIZE, CELL_SIZE};
use render::{RenderState, sync_texture, draw_maze};

/// 应用程序状态
pub struct MazeApp {
    pub map: Map,
    pub original_map: Map,
    pub algorithm: Algorithm,
    pub running: bool,
    pub step_index: usize,
    pub steps: Vec<Point>,
    pub found: bool,
    pub path_length: i32,
    pub start_time: Option<Instant>,
    pub elapsed_ms: u128,
    pub speed: f32, // 毫秒每步
    pub last_update: Instant,
    pub auto_play: bool,
    pub steps_per_frame: usize, // 每帧执行的步数
    pub path_points: Vec<(i32, i32, Direction)>, // 最终路径点及方向
    pub show_arrows: bool, // 是否显示路径箭头
    pub render_state: RenderState,
}

impl MazeApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 配置中文字体
        let mut fonts = egui::FontDefinitions::default();
        
        // 加载系统中文字体 (Windows: Microsoft YaHei)
        fonts.font_data.insert(
            "chinese_font".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "C:\\Windows\\Fonts\\msyh.ttc"
            )),
        );
        
        // 将中文字体添加到字体族
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "chinese_font".to_owned());
        
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "chinese_font".to_owned());
        
        cc.egui_ctx.set_fonts(fonts);
        
        let map = Map::with_size(MAP_SIZE, MAP_SIZE, 
                                  MAP_SIZE * CELL_SIZE as i32, 
                                  MAP_SIZE * CELL_SIZE as i32);
        Self {
            original_map: map.clone(),
            map,
            algorithm: Algorithm::BFS,
            running: false,
            step_index: 0,
            steps: Vec::new(),
            found: false,
            path_length: 0,
            start_time: None,
            elapsed_ms: 0,
            speed: 10.0,
            last_update: Instant::now(),
            auto_play: true,
            steps_per_frame: 1,
            path_points: Vec::new(),
            show_arrows: true,
            render_state: RenderState::default(),
        }
    }
    
    pub fn generate_new_map(&mut self) {
        self.map = Map::with_size(MAP_SIZE, MAP_SIZE,
                                   MAP_SIZE * CELL_SIZE as i32,
                                   MAP_SIZE * CELL_SIZE as i32);
        self.original_map = self.map.clone();
        self.reset_state();
        self.render_state.texture_dirty = true;
    }
    
    pub fn reset_map(&mut self) {
        self.map = self.original_map.clone();
        self.reset_state();
        self.render_state.texture_dirty = true;
    }
    
    fn reset_state(&mut self) {
        self.running = false;
        self.step_index = 0;
        self.steps.clear();
        self.found = false;
        self.path_length = 0;
        self.start_time = None;
        self.elapsed_ms = 0;
        self.last_update = Instant::now();
        self.path_points.clear();
        self.render_state.texture_dirty = true;
    }
    
    pub fn start_search(&mut self) {
        self.map = self.original_map.clone();
        self.running = true;
        self.step_index = 0;
        self.found = false;
        self.path_length = 0;
        self.start_time = Some(Instant::now());
        self.last_update = Instant::now();
        self.render_state.texture_dirty = true;
        
        // 预计算所有步骤
        let mut temp_map = self.map.clone();
        let (steps, found, path_len) = run_algorithm(&mut temp_map, self.algorithm);
        self.steps = steps;
        self.found = found;
        self.path_length = path_len;
        
        // 提取最终路径点及方向
        self.extract_path_directions();
    }
    
    /// 从步骤中提取最终路径及方向
    fn extract_path_directions(&mut self) {
        self.path_points.clear();
        
        // 收集所有 LIGHT_RED 颜色的点（回溯路径）或 AQUAMARINE 颜色的点（最终路径）
        let mut path_steps: Vec<_> = self.steps.iter()
            .filter(|s| s.color == Color::LIGHT_RED || s.color == Color::AQUAMARINE)
            .collect();
        
        if path_steps.is_empty() {
            return;
        }
        
        // 如果是 LIGHT_RED（回溯顺序是从终点到起点），需要反转
        // 检查第一个点的颜色判断
        if !path_steps.is_empty() && path_steps[0].color == Color::LIGHT_RED {
            path_steps.reverse();
        }
        
        // 去重（可能有重复的点）
        let mut seen = std::collections::HashSet::new();
        let unique_steps: Vec<_> = path_steps.into_iter()
            .filter(|s| seen.insert((s.x, s.y)))
            .collect();
        
        // 计算每个点到下一个点的方向
        for i in 0..unique_steps.len() {
            let current = unique_steps[i];
            let direction = if i + 1 < unique_steps.len() {
                let next = unique_steps[i + 1];
                let dx = next.x - current.x;
                let dy = next.y - current.y;
                match (dx, dy) {
                    (1, 0) => Direction::Right,
                    (-1, 0) => Direction::Left,
                    (0, 1) => Direction::Down,
                    (0, -1) => Direction::Up,
                    _ => Direction::None,
                }
            } else {
                Direction::None // 终点没有方向
            };
            
            self.path_points.push((current.x, current.y, direction));
        }
    }
    
    fn update(&mut self) {
        if self.running && self.auto_play && self.step_index < self.steps.len() {
            let elapsed = self.last_update.elapsed().as_millis() as f32;
            if elapsed >= self.speed {
                // 每帧执行多步以提高速度
                for _ in 0..self.steps_per_frame {
                    if self.step_index < self.steps.len() {
                        self.advance_step_fast();
                    } else {
                        break;
                    }
                }
                self.last_update = Instant::now();
            }
        }
    }
    
    /// 快速步进，不更新时间（用于批量执行）
    #[inline(always)]
    fn advance_step_fast(&mut self) {
        let step = &self.steps[self.step_index];
        
        // 使用 match 替代多个 if-else，编译器可更好优化
        let cell_value = match step.color {
            Color::YELLOW => 4,
            Color::SKY_BLUE => 2,
            Color::LIGHT_RED => 3,
            Color::AQUAMARINE => 5,
            _ => self.map.get_cell(step.x, step.y),
        };
        
        self.map.set_cell(step.x, step.y, cell_value);
        self.step_index += 1;
        self.render_state.texture_dirty = true;
        
        if self.step_index >= self.steps.len() {
            self.running = false;
            if let Some(start) = self.start_time {
                self.elapsed_ms = start.elapsed().as_millis();
            }
        }
    }
    
    pub fn advance_step(&mut self) {
        if self.step_index < self.steps.len() {
            self.advance_step_fast();
            
            if let Some(start) = self.start_time {
                self.elapsed_ms = start.elapsed().as_millis();
            }
        }
    }
    
    pub fn step_backward(&mut self) {
        if self.step_index > 0 {
            self.step_index -= 1;
            // 重新应用所有步骤到当前索引
            self.map = self.original_map.clone();
            for i in 0..self.step_index {
                let step = &self.steps[i];
                let cell_value = if step.color == Color::YELLOW {
                    4
                } else if step.color == Color::SKY_BLUE {
                    2
                } else if step.color == Color::LIGHT_RED {
                    3
                } else if step.color == Color::AQUAMARINE {
                    5
                } else {
                    self.map.get_cell(step.x, step.y)
                };
                self.map.set_cell(step.x, step.y, cell_value);
            }
            self.render_state.texture_dirty = true;
        }
    }
    
}

impl eframe::App for MazeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 更新搜索步骤
        self.update();
        
        // 如果正在运行，持续请求重绘
        if self.running {
            ctx.request_repaint();
        }
        
        ui::top_bar(ctx);
        ui::left_panel(ctx, self);
        ui::right_panel(ctx, self);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                sync_texture(&mut self.render_state, ctx, &self.map);
                draw_maze(
                    &self.render_state,
                    ui,
                    &self.path_points,
                    self.step_index >= self.steps.len(),
                    self.show_arrows,
                );
            });
        });

        // 处理快捷键
        input::handle_hotkeys(ctx, self);
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config::VIEWPORT_WIDTH, config::VIEWPORT_HEIGHT])
            .with_resizable(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "迷宫寻路算法演示 - Rust + egui v1.5.0",
        options,
        Box::new(|cc| Ok(Box::new(MazeApp::new(cc)))),
    )
}
