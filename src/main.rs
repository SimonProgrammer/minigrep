use std::env;
use rust_minigrep::Config;
use rust_minigrep::run as start_app;

#[warn(unused_variables)]

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    start_app(config);
}