use app::*;

mod app;
mod config;
mod handler;
mod media;
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
