use eframe::egui;

pub fn run_ui() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };

    eframe::run_native(
        "Drop Load",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}

#[derive(Default)]
struct App {
    selected: Option<String>,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let files = ui.ctx().input(|i| i.raw.dropped_files.clone());
        for file in files {
            let path = file.path().display().to_string();
            self.selected = Some(path);
        }
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            ui.heading("Drag a file here or pick file");
            ui.add_space(10.0);
            if ui.button("Choose file...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    let path = path.display().to_string();
                    self.selected = Some(path);
                }
            }
            if let Some(file_name) = &self.selected {
                let text = format!("Selected: {:?}", file_name);
                ui.heading(text);
            }
        });
    }
}
