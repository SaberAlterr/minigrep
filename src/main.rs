use std::env;
use minigrep::Config;
use minigrep::run;
fn main() {
    let config= Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
