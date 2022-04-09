#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, epi};

/* ### MAIN ### */
fn main() {
    let app = App::default();
    let options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(app), options);
}

/* ### APP ### */
#[derive(Default)]
struct App {}

impl epi::App for App {
    fn name(&self) -> &str {
        "Hello World"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Hello World");
            });

            time_radius(ui);
        });
    }
}

fn time_radius(ui: &mut egui::Ui) {
    use eframe::epaint::Shape;
    use egui::{pos2, vec2, Color32, Pos2, Stroke};

    ui.ctx().request_repaint();

    let available_rect = ui.ctx().available_rect().max;
    let center = pos2(available_rect.x / 2.0, available_rect.y / 2.0);
    let circle_radius = 60.0;
    let wrap_stroke = Stroke::new(1.0, Color32::WHITE);
    let n_points = 60;

    let time = ui.input().time;

    // ### hour ### //

    ui.painter()
        .circle_stroke(center, circle_radius, wrap_stroke);

    let end_angle = time / 12.0 * 360_f64.to_radians();

    let points: Vec<Pos2> = (0..n_points)
        .map(|i| {
            let angle = egui::lerp(0.0..=end_angle, i as f64 / n_points as f64);
            let (sin, cos) = angle.sin_cos();
            center + circle_radius * vec2(sin as f32, -cos as f32)
        })
        .collect();

    ui.painter()
        .add(Shape::line(points, Stroke::new(4.0, Color32::BLUE)));
}
