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

        // 文字の大きさを変更
        // ctx.set_pixels_per_point(1.5);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("NotoSansJP");
            ui.label("Hello World ハローワールド");
        });
    }
}
