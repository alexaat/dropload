mod model;
mod server;
mod ui;
use server::start_server;
use std::sync::Arc;
use std::sync::Mutex;
use ui::run_ui;

use uuid::Uuid;

// #[derive(Default)]
// pub struct Link {
//     file_path: Option<String>,
//     link: Option<String>,
// }

#[derive(Debug)]
pub struct App {
    host: Option<String>,
    file_path: Option<String>,
    file_name: Option<String>,
    link: Option<String>,
}

impl App {
    pub fn set_file_path(&mut self, file_path: String) {
        self.file_path = Some(file_path.clone());
        let host = self.host.as_ref().expect("no host...");
        self.link = Some(format!("{}/{}", host, Uuid::now_v7()));
        self.file_name = Some(file_path.split("/").last().unwrap().to_string());
    }
}

fn main() {
    let app = App {
        host: None,
        file_name: None,
        file_path: None,
        link: None,
    };
    let app = Arc::new(Mutex::new(app));
    start_server(Arc::clone(&app));
    run_ui(app).expect("fail to start application");
}
