use std::path::PathBuf;
use crate::Cli;

pub struct Config {
    pub query: String,
    pub filename: PathBuf,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: Cli) -> Result<Config, &'static str> {
        if false {
            return Err("not enough arguments");            
        }
        
        Ok(Config{ 
            filename: args.filepath,
            query: args.pattern, 
            case_sensitive: args.case 
        })
    }
}