#[macro_use]
extern crate clap;

mod app;
use app::build_app;
fn main() {
    let app = build_app();
    println!("{:?}", app.get_matches())
}
