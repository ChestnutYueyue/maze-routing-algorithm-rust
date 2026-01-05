use eframe::egui;
use crate::{config::{MAP_SIZE, CELL_SIZE, CELL_COLORS}, Map, Direction};

pub struct RenderState {
    pub texture: Option<egui::TextureHandle>,
    pub texture_dirty: bool,
}

impl Default for RenderState {
    fn default() -> Self {
        Self { texture: None, texture_dirty: true }
    }
}

/// 将当前地图同步到 GPU 纹理
pub fn sync_texture(state: &mut RenderState, ctx: &egui::Context, map: &Map) {
    if !state.texture_dirty {
        return;
    }
    let w = (MAP_SIZE + 1) as usize;
    let h = (MAP_SIZE + 1) as usize;
    let mut image = egui::ColorImage::new([w, h], egui::Color32::BLACK);
    for y in 0..h {
        for x in 0..w {
            let v = map.grid[y][x] as usize;
            let idx = if v < CELL_COLORS.len() { v } else { 0 };
            image[(x, y)] = CELL_COLORS[idx];
        }
    }
    let options = egui::TextureOptions::NEAREST;
    if let Some(tex) = state.texture.as_mut() {
        tex.set(image, options);
    } else {
        state.texture = Some(ctx.load_texture("maze_grid_texture", image, options));
    }
    state.texture_dirty = false;
}

/// 绘制地图纹理和路径箭头
pub fn draw_maze(
    state: &RenderState,
    ui: &mut egui::Ui,
    path_points: &[(i32, i32, Direction)],
    steps_finished: bool,
    show_arrows: bool,
) {
    let (response, painter) = ui.allocate_painter(
        egui::vec2((MAP_SIZE + 1) as f32 * CELL_SIZE, (MAP_SIZE + 1) as f32 * CELL_SIZE),
        egui::Sense::hover()
    );

    let rect = response.rect;
    let base_x = rect.min.x;
    let base_y = rect.min.y;

    if let Some(tex) = state.texture.as_ref() {
        let tex_id = tex.id();
        let rect = egui::Rect::from_min_size(
            egui::pos2(base_x, base_y),
            egui::vec2((MAP_SIZE + 1) as f32 * CELL_SIZE, (MAP_SIZE + 1) as f32 * CELL_SIZE)
        );
        painter.image(
            tex_id,
            rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }

    // 绘制路径方向虚线箭头
    if show_arrows && steps_finished && path_points.len() > 1 {
        let line_color = egui::Color32::from_rgb(139, 69, 19);
        let stroke = egui::Stroke::new(1.5, line_color);
        for i in 0..path_points.len() - 1 {
            let (x1, y1, _) = path_points[i];
            let (x2, y2, _) = path_points[i + 1];

            let start = egui::pos2(
                base_x + (x1 as f32 + 0.5) * CELL_SIZE,
                base_y + (y1 as f32 + 0.5) * CELL_SIZE
            );
            let end = egui::pos2(
                base_x + (x2 as f32 + 0.5) * CELL_SIZE,
                base_y + (y2 as f32 + 0.5) * CELL_SIZE
            );

            let dx = end.x - start.x;
            let dy = end.y - start.y;
            let len = (dx * dx + dy * dy).sqrt();
            let dash_len = 3.0;
            let gap_len = 2.0;
            let total_segment = dash_len + gap_len;

            let mut t = 0.0;
            while t < len {
                let t_start = t / len;
                let t_end = ((t + dash_len) / len).min(1.0);

                let p1 = egui::pos2(start.x + dx * t_start, start.y + dy * t_start);
                let p2 = egui::pos2(start.x + dx * t_end, start.y + dy * t_end);

                painter.line_segment([p1, p2], stroke);
                t += total_segment;
            }

            if i % 3 == 0 {
                let (_, _, direction) = path_points[i];
                let mid = egui::pos2((start.x + end.x) / 2.0, (start.y + end.y) / 2.0);
                let arrow_size = 3.0;
                let (arrow_dx, arrow_dy): (f32, f32) = match direction {
                    Direction::Up => (0.0, -1.0),
                    Direction::Down => (0.0, 1.0),
                    Direction::Left => (-1.0, 0.0),
                    Direction::Right => (1.0, 0.0),
                    Direction::None => (0.0, 0.0),
                };

                if arrow_dx != 0.0 || arrow_dy != 0.0 {
                    let tip = egui::pos2(mid.x + arrow_dx * arrow_size, mid.y + arrow_dy * arrow_size);
                    let left = egui::pos2(
                        mid.x - arrow_dy * arrow_size * 0.5 - arrow_dx * arrow_size,
                        mid.y + arrow_dx * arrow_size * 0.5 - arrow_dy * arrow_size,
                    );
                    let right = egui::pos2(
                        mid.x + arrow_dy * arrow_size * 0.5 - arrow_dx * arrow_size,
                        mid.y - arrow_dx * arrow_size * 0.5 - arrow_dy * arrow_size,
                    );

                    painter.add(egui::Shape::convex_polygon(vec![tip, left, right], line_color, egui::Stroke::NONE));
                }
            }
        }
    }
}
