use std::process::exit;

mod dumbgrep;
// mod errors;

// pub use errors::MiniGrepError;
use dumbgrep::{Config, run};

fn main() {

    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("There is a problem parsing the arguments: {err}");
        exit(1);
    });
    
    if let Err(e) = run(&config) {
        eprintln!("Failed to query the text, error: {}", e);
        exit(1);
    }
    
}
