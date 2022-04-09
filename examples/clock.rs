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
                ui.heading("Clock");
            });

            clock(ui);
        });
    }
}

fn clock(ui: &mut egui::Ui) {
    use chrono::*;
    use eframe::epaint::Shape;
    use egui::{pos2, vec2, Color32, Pos2, Rect, Stroke};

    ui.ctx().request_repaint();

    let available_rect = ui.ctx().available_rect().max;
    let center = pos2(available_rect.x / 2.0, available_rect.y / 2.0);
    let circle_radius = 60.0;
    let wrap_stroke = Stroke::new(1.0, Color32::WHITE);
    let n_points = 60;
    let text_rect_size = vec2(120.0, 120.0);

    // 時間
    let utc: DateTime<Utc> = Utc::now();
    let hour = utc.hour() as f64 + 9.0;
    let minute = utc.minute() as f64;
    let second = utc.second() as f64;

    // ### hour ### //
    let hour_center = center - vec2(circle_radius * 3.0, 0.0);

    ui.painter()
        .circle_stroke(hour_center, circle_radius, wrap_stroke);

    let hour = if hour >= 12.0 { hour - 12.0 } else { hour };
    let end_angle = hour / 12.0 * 360_f64.to_radians();

    let points: Vec<Pos2> = (0..n_points)
        .map(|i| {
            let angle = egui::lerp(0.0..=end_angle, i as f64 / n_points as f64);
            let (sin, cos) = angle.sin_cos();
            hour_center + circle_radius * vec2(sin as f32, -cos as f32)
        })
        .collect();

    let hour_rect = Rect::from_center_size(hour_center, text_rect_size);
    ui.allocate_ui_at_rect(hour_rect, |ui| {
        ui.centered_and_justified(|ui| {
            ui.heading(format!("{}", &hour));
        });
    });

    ui.painter()
        .add(Shape::line(points, Stroke::new(4.0, Color32::RED)));

    // ### minute ### //
    let minute_center = center;

    ui.painter()
        .circle_stroke(minute_center, circle_radius, wrap_stroke);

    let end_angle = minute / 60.0 * 360_f64.to_radians();

    let points: Vec<Pos2> = (0..n_points)
        .map(|i| {
            let angle = egui::lerp(0.0..=end_angle, i as f64 / n_points as f64);
            let (sin, cos) = angle.sin_cos();
            minute_center + circle_radius * vec2(sin as f32, -cos as f32)
        })
        .collect();

    let minute_rect = Rect::from_center_size(minute_center, text_rect_size);
    ui.allocate_ui_at_rect(minute_rect, |ui| {
        ui.centered_and_justified(|ui| {
            ui.heading(format!("{}", &minute));
        });
    });

    ui.painter()
        .add(Shape::line(points, Stroke::new(4.0, Color32::GREEN)));

    // ### second ### //
    let second_center = center + vec2(circle_radius * 3.0, 0.0);

    ui.painter()
        .circle_stroke(second_center, circle_radius, wrap_stroke);

    let end_angle = second / 60.0 * 360_f64.to_radians();

    let points: Vec<Pos2> = (0..n_points)
        .map(|i| {
            let angle = egui::lerp(0.0..=end_angle, i as f64 / n_points as f64);
            let (sin, cos) = angle.sin_cos();
            second_center + circle_radius * vec2(sin as f32, -cos as f32)
        })
        .collect();

    let second_rect = Rect::from_center_size(second_center, text_rect_size);
    ui.allocate_ui_at_rect(second_rect, |ui| {
        ui.centered_and_justified(|ui| {
            ui.heading(format!("{}", &second));
        });
    });

    ui.painter()
        .add(Shape::line(points, Stroke::new(4.0, Color32::BLUE)));
}
