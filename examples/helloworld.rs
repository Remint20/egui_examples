#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, epi};

fn main() {
    let app = App::default();
    let options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(app), options);
}

#[derive(Default)]
struct App {}

impl epi::App for App {
    fn name(&self) -> &str {
        "Hello World"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World");
        });
    }
}
