mod server;
mod ui;
use server::start_server;
use ui::run_ui;

fn main() {
    start_server();
    run_ui().expect("fail to start application");
}
