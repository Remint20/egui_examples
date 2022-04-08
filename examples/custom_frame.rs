use eframe::{egui, epi};

/* ### MAIN ### */
fn main() {
    let app = CustomFrame::default();
    let options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(app), options);
}

/* ### APP ### */
#[derive(Default)]
struct CustomFrame {}

impl epi::App for CustomFrame {
    fn name(&self) -> &str {
        "Hello World"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Examples");

            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.label("popup");
            });

            egui::Frame::menu(ui.style()).show(ui, |ui| {
                ui.label("Menu");
            });

            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.label("group");
            });

            egui::Frame::none().show(ui, |ui| {
                ui.label("none");
            });

            egui::Frame::window(ui.style()).show(ui, |ui| {
                ui.label("window");
            });

            egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
                ui.label("dark_canvas");
            });

            egui::Frame::default()
                .fill(egui::Color32::DARK_RED)
                .stroke(egui::Stroke::new(2.0, egui::Color32::WHITE))
                .rounding(2.0)
                .margin(egui::style::Margin::same(20.0))
                .shadow(eframe::epaint::Shadow {
                    extrusion: 10.0,
                    color: egui::Color32::LIGHT_YELLOW,
                })
                .multiply_with_opacity(0.5)
                .show(ui, |ui| {
                    ui.label("custom frame");
                });
        });
    }
}
