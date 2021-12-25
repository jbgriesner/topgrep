use std::path::PathBuf;
use crate::Cli;

pub struct Config {
    pub query: String,
    pub filename: PathBuf,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &Cli) -> Result<Config, &str> {
        if false {
            return Err("not enough arguments");            
        }
        
        Ok(Config{ 
            filename: args.filepath.clone(),
            query: args.pattern.clone(), 
            case_sensitive: args.case 
        })
    }
}