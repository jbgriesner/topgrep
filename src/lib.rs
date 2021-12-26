use std::error::Error;
use std::fs;

mod config;
pub use config::Config;

mod args;
pub use args::Cli;

mod algo;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        algo::search(&config.query, &contents)
    } else {
        algo::search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
