use eframe::egui;

pub const MAP_SIZE: i32 = 56;
pub const CELL_SIZE: f32 = 10.0;

// GUI 布局尺寸
pub const VIEWPORT_WIDTH: f32 = 1100.0;
pub const VIEWPORT_HEIGHT: f32 = 700.0;

// 预定义颜色查找表
pub const CELL_COLORS: [egui::Color32; 6] = [
    egui::Color32::WHITE,                    // 0 - 通道
    egui::Color32::from_rgb(0, 10, 0),       // 1 - 墙壁
    egui::Color32::from_rgb(135, 206, 235),  // 2 - 已访问 (天蓝)
    egui::Color32::from_rgb(255, 106, 106),  // 3 - 回溯 (浅红)
    egui::Color32::from_rgb(255, 255, 0),    // 4 - 起点/终点 (黄)
    egui::Color32::from_rgb(127, 255, 212),  // 5 - 最终路径 (青)
];
