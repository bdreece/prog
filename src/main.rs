use std::path::PathBuf;
use std::string::String;
use std::vec::Vec;

use clap::{ArgEnum, Parser};

#[derive(Parser, Debug)]
#[clap(
    author = "Brian Reece",
    version = "v0.3-alpha",
    about = "A tool for centralizing scripted commands via a configurable markup file",
    long_about = None
)]
struct Args {
    #[clap(
        short,
        long,
        parse(from_os_str),
        value_name = "PATH",
        help = "Path to config file"
    )]
    path: Option<PathBuf>,

    #[clap(
        short,
        long,
        arg_enum,
        group = "format_group",
        value_name = "FORMAT",
        help = "Config file format"
    )]
    format: Option<Format>,

	#[clap(
        short,
        long,
        arg_enum,
        group = "template_group",
        value_name = "TEMPLATE",
        help = "Config file template"
    )]
	template: Option<Template>,

    #[clap(short, long, parse(from_occurrences), help = "Sets verbosity level")]
    verbose: usize,

	#[clap(short, long, requires = "format_group", requires = "template_group", help = "Generate config file")]
    generate: bool,

    targets: Vec<String>,
}

#[derive(ArgEnum, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Format {
    Json,
    Ron,
    Toml,
    Url,
    Xml,
    Yaml,
}

#[derive(ArgEnum, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Template {
	Cmake,
    Cargo,
    Go,
    Node,
	Make,
    Python,
}

fn main() {
    let args = Args::parse();

    if let Some(format) = args.format {
		println!("{:?}", format);
    }

    if let Some(template) = args.template {
    	println!("{:?}", template);
    }

    println!("{}", args.verbose);
}
