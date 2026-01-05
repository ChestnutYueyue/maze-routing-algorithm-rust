use eframe::egui;
use crate::MazeApp;

pub fn handle_hotkeys(ctx: &egui::Context, app: &mut MazeApp) {
    if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
        if app.steps.is_empty() {
            app.start_search();
        } else {
            app.auto_play = !app.auto_play;
        }
    }
    if ctx.input(|i| i.key_pressed(egui::Key::R)) {
        app.reset_map();
    }
    if ctx.input(|i| i.key_pressed(egui::Key::N)) {
        app.generate_new_map();
    }
    if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
        app.step_backward();
    }
    if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
        app.advance_step();
    }
}
