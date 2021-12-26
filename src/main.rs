use std::process;
use structopt::StructOpt;
use topgrep::{Cli, Config};

fn main() {
    let args = Cli::from_args();

    let config = Config::new(args).unwrap_or_else(
        |err| {
            eprintln!("Parsing error: {}", err);
            process::exit(1);
        }
    );

    if let Err(e) = topgrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }    
}


