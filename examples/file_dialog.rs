use eframe::{egui, epi};

/* ### MAIN ### */
fn main() {
    let app = FileDialog::default();
    let options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(app), options);
}

/* ### APP ### */
#[derive(Default)]
struct FileDialog {}

impl epi::App for FileDialog {
    fn name(&self) -> &str {
        "Hello World"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Examples");

            ui.hyperlink_to("Dialog rfd", "https://crates.io/crates/rfd");

            if ui.button("Open folder...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    println!("{:?}", path);
                }
            }
            if ui.button("Open file...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    println!("{:?}", path);
                }
            }
            if ui.button("Open files...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_files() {
                    println!("{:?}", path);
                }
            }

            if ui.button("Save file...").clicked() {
                if let Some(path) = rfd::FileDialog::new().save_file() {
                    println!("{:?}", path);
                }
            }
        });
    }
}
