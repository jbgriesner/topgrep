use std::env;
use std::process;

use topgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(
        |err| {
            println!("Parsing error: {}", err);
            process::exit(1);
        }
    );

    if let Err(e) = topgrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }    
}


