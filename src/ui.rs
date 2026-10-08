use crate::App;
use eframe::egui;
use std::sync::Arc;
use std::sync::Mutex;

struct GuiApp {
    app: Arc<Mutex<App>>,
}

pub fn run_ui(app: Arc<Mutex<App>>) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };

    let gui_app = GuiApp { app };

    eframe::run_native("Drop Load", options, Box::new(|_cc| Ok(Box::new(gui_app))))
}

impl eframe::App for GuiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let files = ui.ctx().input(|i| i.raw.dropped_files.clone());
        for file in files {
            let path = file.path().display().to_string();
            self.app.lock().unwrap().set_file_path(path);
            //app.set_file_path(path);
        }
        ui.vertical_centered(|ui| {
            let info = format!("{:?}", self.app.lock().unwrap());
            ui.heading(format!("info: {}", info));
            ui.add_space(100.0);
            ui.heading("Drag a file here or pick file");
            ui.add_space(10.0);
            if ui.button("Choose file...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    let path = path.display().to_string();
                    self.app.lock().unwrap().set_file_path(path);
                }
            }
            if let Some(file_name) = &self.app.lock().unwrap().file_name {
                let text = format!("file name: {:?}", file_name);
                ui.heading(text);
            }
            if let Some(file_path) = &self.app.lock().unwrap().file_path {
                let text = format!("file path: {:?}", file_path);
                ui.heading(text);
            }

            if let Some(link) = &self.app.lock().unwrap().link {
                ui.add_space(10.0);
                let text = format!("{:?}", link);
                ui.heading("use link to download file:");
                ui.add_space(10.0);
                ui.heading(text);
            }
        });
    }
}
