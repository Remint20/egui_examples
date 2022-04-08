use eframe::{egui, epi};

/* ### MAIN ### */
fn main() {
    let app = ExampleApp::default();
    let options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(app), options);
}

/* ### APP ### */
#[derive(Default)]
struct ExampleApp {}

impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "Hello World"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Examples");
        });
    }
}
