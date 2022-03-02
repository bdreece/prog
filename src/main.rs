use std::path::PathBuf;
use std::string::String;
use std::vec::Vec;

use clap::{ArgEnum, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author = "Brian Reece", version = "v0.3-alpha", about = "A tool for centralizing scripted commands via a configurable markup file", long_about = None)]
struct Args {
	#[clap(short, long, parse(from_os_str), value_name = "PATH", help = "Path to config file")]
    path: Option<PathBuf>,

	#[clap(short, long, arg_enum, value_name = "FORMAT", help = "Config file format")]
    format: Option<Format>,

    #[clap(subcommand)]
    command: Command,

	#[clap(short, long, parse(from_occurrences), help = "Sets verbosity level")]
    verbose: usize,

    #[clap(last = true)]
    targets: Vec<String>
}

#[derive(ArgEnum, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Format {
	Json,
    Toml,
    Yaml,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generates a config file
    Generate,
    /// Edits the config file
    Edit,
}

fn main() {
	let _args = Args::parse();

    
	
}
