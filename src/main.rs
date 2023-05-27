use app::*;
use ui::*;

mod app;
mod config;
mod ui;
mod media;
mod handler;

fn main() {
    let mut app = App::new().unwrap();
    match app.run() {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
