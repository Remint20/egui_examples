#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, epi};

#[derive(Default)]
struct GalaxyApp {}

impl epi::App for GalaxyApp {
    fn name(&self) -> &str {
        "galaxy aniamtion"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint();

            let time = ui.input().time;

            let window_rect = ctx.available_rect();
            let window_width = window_rect.max.x;
            let window_height = window_rect.max.y;

            // 中心
            let center = egui::pos2(window_width / 2.0, window_height / 2.0);

            // 太陽
            let sun_radius = 40.0;
            ui.painter()
                .circle_filled(center, sun_radius, egui::Color32::WHITE);

            // 水星
            let mercury_radius = 1.0;
            let mercury_distance = 60.0;
            let speed = 5.0;
            let mercury_x = (time * speed).sin() as f32 * mercury_distance + center.x;
            let mercury_y = (time * speed).cos() as f32 * mercury_distance + center.y;
            ui.painter().circle_filled(
                egui::pos2(mercury_x, mercury_y),
                mercury_radius,
                egui::Color32::WHITE,
            );
            ui.painter().circle_stroke(
                center,
                mercury_distance,
                egui::Stroke::new(0.1, egui::Color32::WHITE),
            );

            // 金星
            let venus_radius = 10.0;
            let venus_distance = 80.0;
            let speed = 2.0;
            let venus_x = (time * speed).sin() as f32 * venus_distance + center.x;
            let venus_y = (time * speed).cos() as f32 * venus_distance + center.y;
            ui.painter().circle_filled(
                egui::pos2(venus_x, venus_y),
                venus_radius,
                egui::Color32::WHITE,
            );
            ui.painter().circle_stroke(
                center,
                venus_distance,
                egui::Stroke::new(0.1, egui::Color32::WHITE),
            );

            // 地球
            let earth_radius = 12.0;
            let earth_distance = 120.0;
            let earth_x = (time + std::f64::consts::PI).sin() as f32 * earth_distance + center.x;
            let earth_y = (time + std::f64::consts::PI).cos() as f32 * earth_distance + center.y;
            ui.painter().circle_filled(
                egui::pos2(earth_x, earth_y),
                earth_radius,
                egui::Color32::WHITE,
            );
            ui.painter().circle_stroke(
                center,
                earth_distance,
                egui::Stroke::new(0.1, egui::Color32::WHITE),
            );

            // 火星
            let mars_radius = 6.0;
            let mars_distance = 160.0;
            let mars_x = (time + std::f64::consts::PI).sin() as f32 * mars_distance + center.x;
            let mars_y = time.cos() as f32 * mars_distance + center.y;
            ui.painter().circle_filled(
                egui::pos2(mars_x, mars_y),
                mars_radius,
                egui::Color32::WHITE,
            );
            ui.painter().circle_stroke(
                center,
                mars_distance,
                egui::Stroke::new(0.1, egui::Color32::WHITE),
            );

            // 木星
            let jupiter_radius = 20.0;
            let jupiter_distance = 230.0;
            let jupiter_x = time.sin() as f32 * jupiter_distance + center.x;
            let jupiter_y =
                (time + std::f64::consts::PI).cos() as f32 * jupiter_distance + center.y;
            ui.painter().circle_filled(
                egui::pos2(jupiter_x, jupiter_y),
                jupiter_radius,
                egui::Color32::WHITE,
            );
            ui.painter().circle_stroke(
                center,
                jupiter_distance,
                egui::Stroke::new(0.1, egui::Color32::WHITE),
            );
            // 土星
            let saturn_radius = 17.0;
            let saturn_distance = 300.0;
            let saturn_x = time.sin() as f32 * saturn_distance + center.x;
            let saturn_y = time.cos() as f32 * saturn_distance + center.y;
            ui.painter().circle_filled(
                egui::pos2(saturn_x, saturn_y),
                saturn_radius,
                egui::Color32::WHITE,
            );
            ui.painter().circle_stroke(
                center,
                saturn_distance,
                egui::Stroke::new(0.1, egui::Color32::WHITE),
            );
            // 天王星
            let uranus_radius = 14.0;
            let uranus_distance = 400.0;
            let uranus_x = time.sin() as f32 * uranus_distance + center.x;
            let uranus_y = time.cos() as f32 * uranus_distance + center.y;
            ui.painter().circle_filled(
                egui::pos2(uranus_x, uranus_y),
                uranus_radius,
                egui::Color32::WHITE,
            );
            ui.painter().circle_stroke(
                center,
                uranus_distance,
                egui::Stroke::new(0.1, egui::Color32::WHITE),
            );
            // 海王星
            let neptune_radius = 13.4;
            let neptune_distance = 600.0;
            let neptune_x = time.sin() as f32 * neptune_distance + center.x;
            let neptune_y = time.cos() as f32 * neptune_distance + center.y;
            ui.painter().circle_filled(
                egui::pos2(neptune_x, neptune_y),
                neptune_radius,
                egui::Color32::WHITE,
            );
            ui.painter().circle_stroke(
                center,
                neptune_distance,
                egui::Stroke::new(0.1, egui::Color32::WHITE),
            );
        });
    }
}

fn main() {
    let app = GalaxyApp::default();
    let options = eframe::NativeOptions {
        initial_window_pos: Some(egui::pos2(920., 0.)),
        initial_window_size: Some(egui::vec2(1000., 1000.)),
        ..Default::default()
    };

    eframe::run_native(Box::new(app), options);
}
