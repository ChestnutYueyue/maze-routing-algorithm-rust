use crate::{Algorithm, MazeApp, config::{MAP_SIZE, CELL_SIZE}};
use eframe::egui;

pub fn top_bar(ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.heading("🔍 迷宫寻路算法演示 (Rust + egui v1.5.0)");
        ui.separator();
    });
}

pub fn left_panel(ctx: &egui::Context, app: &mut MazeApp) {
    let canvas_height = (MAP_SIZE + 1) as f32 * CELL_SIZE;
    egui::SidePanel::left("left_panel").min_width(230.0).show(ctx, |ui| {
        ui.set_max_height(canvas_height);
        egui::ScrollArea::vertical().max_height(canvas_height).show(ui, |ui| {
            ui.heading("控制");
            ui.separator();

            ui.group(|ui| {
                ui.label("选择算法:");
                if ui.add(egui::RadioButton::new(app.algorithm == Algorithm::DFS, "🔸 DFS (深度优先)")).clicked() {
                    app.algorithm = Algorithm::DFS;
                }
                if ui.add(egui::RadioButton::new(app.algorithm == Algorithm::BFS, "🔹 BFS (广度优先)")).clicked() {
                    app.algorithm = Algorithm::BFS;
                }
                if ui.add(egui::RadioButton::new(app.algorithm == Algorithm::DBFS, "🔷 DBFS (双向BFS)")).clicked() {
                    app.algorithm = Algorithm::DBFS;
                }
                if ui.add(egui::RadioButton::new(app.algorithm == Algorithm::AStar, "⭐ A* (启发式)")).clicked() {
                    app.algorithm = Algorithm::AStar;
                }
            });

            ui.separator();

            ui.group(|ui| {
                ui.label("操作:");
                if ui.button("▶ 开始搜索").clicked() {
                    app.start_search();
                }
                if ui.button("⏸ 暂停/继续").clicked() {
                    app.auto_play = !app.auto_play;
                }
                ui.horizontal(|ui| {
                    if ui.button("⏮ 上一步").clicked() && !app.steps.is_empty() {
                        app.step_backward();
                    }
                    if ui.button("⏭ 下一步").clicked() && !app.steps.is_empty() {
                        app.advance_step();
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("🔄 重置地图").clicked() {
                        app.reset_map();
                    }
                    if ui.button("🎲 生成新地图").clicked() {
                        app.generate_new_map();
                    }
                });
            });

            ui.separator();

            ui.group(|ui| {
                ui.label("播放参数:");
                ui.label("动画速度 (ms/步):");
                ui.add(egui::Slider::new(&mut app.speed, 1.0..=100.0).step_by(1.0));
                ui.label(format!("当前: {:.0} ms", app.speed));

                ui.separator();
                ui.label("每帧步数:");
                ui.add(egui::Slider::new(&mut app.steps_per_frame, 1..=50));
                ui.label(format!("当前: {} 步/帧", app.steps_per_frame));

                ui.separator();
                ui.checkbox(&mut app.show_arrows, "🔀 显示路径箭头");
            });
        });
    });
}

pub fn right_panel(ctx: &egui::Context, app: &mut MazeApp) {
    let canvas_height = (MAP_SIZE + 1) as f32 * CELL_SIZE;
    egui::SidePanel::right("right_panel").min_width(230.0).show(ctx, |ui| {
        ui.set_max_height(canvas_height);
        egui::ScrollArea::vertical().max_height(canvas_height).show(ui, |ui| {
            ui.heading("信息");
            ui.separator();

            ui.group(|ui| {
                ui.label("📊 统计信息:");
                ui.label(format!("算法: {}", app.algorithm.short_name()));
                ui.label(format!("状态: {}", if app.running { "运行中" } else { "已停止" }));
                ui.label(format!("自动播放: {}", if app.auto_play { "是" } else { "否" }));
                ui.label(format!("当前步骤: {} / {}", app.step_index, app.steps.len()));
                ui.label(format!("找到路径: {}", if app.found { "是" } else { "否" }));
                if app.found {
                    ui.label(format!("路径长度: {}", app.path_length));
                }
                ui.label(format!("耗时: {} ms", app.elapsed_ms));
            });

            ui.separator();

            ui.group(|ui| {
                ui.label("🎨 颜色说明:");
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(0, 10, 0), "■");
                    ui.label("墙壁");
                });
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::WHITE, "■");
                    ui.label("通道");
                });
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(255, 255, 0), "■");
                    ui.label("起点/终点");
                });
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(135, 206, 235), "■");
                    ui.label("已访问");
                });
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(255, 106, 106), "■");
                    ui.label("回溯");
                });
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(127, 255, 212), "■");
                    ui.label("最终路径");
                });
            });

            ui.separator();

            ui.group(|ui| {
                ui.label("⌨ 快捷键:");
                ui.label("空格: 开始/暂停");
                ui.label("R: 重置地图");
                ui.label("N: 生成新地图");
                ui.label("←/→: 上一步/下一步");
            });
        });
    });
}
