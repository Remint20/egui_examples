#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, epi};

fn main() {
    let app = CustomFont::default();
    let options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(app), options);
}

#[derive(Default)]
struct CustomFont {}

impl epi::App for CustomFont {
    fn name(&self) -> &str {
        "custom_font"
    }

    fn setup(
        &mut self,
        ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        /* フォントの設定 */
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/fonts/NotoSansJP-Medium.otf")),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .push("my_font".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("my_font".to_owned());

        ctx.set_fonts(fonts);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("NotoSansJP");
            ui.label("Hello World ハローワールド");
        });
    }
}
