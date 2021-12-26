use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "topgrep", about = "A grep on steroids.", setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Cli {
    /// pattern to look for
    #[structopt(short = "p", long = "pattern")]
    pub pattern: String,

    /// Input file path
    #[structopt(parse(from_os_str), short = "f", long = "filepath")]
    pub filepath: PathBuf,

    /// Case sensitive
    #[structopt(short = "c", long = "case-sensitive")]
    pub case: bool,
}
