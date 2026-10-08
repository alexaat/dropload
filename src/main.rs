mod model;
mod server;
mod ui;
use server::start_server;
use ui::run_ui;

use uuid::Uuid;

// #[derive(Default)]
// pub struct Link {
//     file_path: Option<String>,
//     link: Option<String>,
// }

pub struct App {
    host: String,
    file_path: Option<String>,
    file_name: Option<String>,
    link: Option<String>,
}

impl App {
    pub fn set_file_path(&mut self, file_path: String) {
        self.file_path = Some(file_path);
        self.link = Some(format!("{}/{}", self.host, Uuid::now_v7()));
    }
}

fn main() {
    let host = start_server();
    let app = App {
        host,
        file_name: None,
        file_path: None,
        link: None,
    };
    run_ui(Box::new(app)).expect("fail to start application");
}
