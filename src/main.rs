mod server;
mod ui;
use qrcode_generator::qr::{Encoder, ErrorCorrection};
use server::start_server;
use std::sync::Arc;
use std::sync::Mutex;
use ui::run_ui;
use uuid::Uuid;

#[derive(Debug)]
pub struct App {
    host: Option<String>,
    file_path: Option<String>,
    file_name: Option<String>,
    link: Option<String>,
    qrcode: Option<Vec<Vec<bool>>>,
    error_message: Option<String>,
}

impl App {
    pub fn set_file_path(&mut self, file_path: String) -> Result<(), String> {
        self.error_message = None;
        self.file_path = Some(file_path.clone());
        let host = self.host.as_ref().ok_or("cannot get host...")?;
        self.link = Some(format!("{}/{}", host, Uuid::now_v7()));
        self.file_name = Some(file_path.split("/").last().unwrap().to_string());

        let symbol = Encoder::new(ErrorCorrection::Medium)
            .encode_text(self.link.as_ref().ok_or("cannot get link...")?)
            .map_err(|e| format!("qrcode map error.../r/n{}", e))?;
        self.qrcode = Some(symbol.to_matrix());
        Ok(())
    }
}

fn main() {
    let app = App {
        host: None,
        file_name: None,
        file_path: None,
        link: None,
        qrcode: None,
        error_message: Some("Error".to_string()),
    };
    let app = Arc::new(Mutex::new(app));
    if let Err(error_message) = start_server(Arc::clone(&app)) {
        match app.lock() {
            Ok(mut lock) => lock.error_message = Some(error_message),
            Err(e) => eprintln!("cannot access app data...\r\n{}", e),
        }
    }
    run_ui(app).expect("fail to start application...");
}
