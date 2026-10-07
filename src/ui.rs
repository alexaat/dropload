use crate::App;
use eframe::egui;

pub fn run_ui(app: Box<App>) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };

    eframe::run_native("Drop Load", options, Box::new(|_cc| Ok(app)))
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let files = ui.ctx().input(|i| i.raw.dropped_files.clone());
        for file in files {
            let path = file.path().display().to_string();
            self.set_file_path(path);
        }
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            ui.heading("Drag a file here or pick file");
            ui.add_space(10.0);
            if ui.button("Choose file...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    let path = path.display().to_string();
                    self.set_file_path(path);
                }
            }
            if let Some(file_name) = &self.file_path {
                let text = format!("Selected: {:?}", file_name);
                ui.heading(text);
            }
            if let Some(link) = &self.link {
                ui.add_space(10.0);
                let text = format!("{:?}", link);
                ui.heading("use link to download file:");
                ui.add_space(10.0);
                ui.heading(text);
            }
        });
    }
}
