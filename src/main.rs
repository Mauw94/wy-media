use app::*;
use ui::*;

mod app;
mod config;
mod ui;

fn main() {
    let mut app = App::new().unwrap();
    match app.run() {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
