use lilgrep::Config;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("[Error] Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lilgrep::run(config) {
        println!("[Error]: {e}");
        process::exit(1);
    }
}
