use std::ffi::OsStr;
use std::io::Result;
use std::path::PathBuf;
use std::string::String;

use clap::Parser;

mod convert;
mod exec;
mod generate;
mod lex;
mod parse;

use crate::convert::convert_config;
//use crate::exec::exec;
use crate::generate::{generate_config, Format, Template};
//use crate::lex::lex_targets;
use crate::parse::parse_config;

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

    #[clap(
        short,
        long,
        requires = "format_group",
        requires = "template_group",
        help = "Generate config file"
    )]
    generate: bool,

    #[clap(
        short,
        long,
        requires = "format_group",
        help = "Convert between markup formats"
    )]
	convert: bool,

    #[clap(help = "Formatted string specifying command targets")]
    targets: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let path = args.path.unwrap_or(PathBuf::from(OsStr::new(".")));
	let format = args.format.unwrap_or(Format::Yaml);
    let template = args.template.unwrap_or(Template::Bare);

    if args.generate {
        generate_config(
            &path,
            &format,
            template
        )?;
    }

    if args.convert {
    	convert_config(&path, &format)?;
    }

    let config = parse_config(&path)?;

	println!("{:?}", config);

    /*
      if let Some(targets) = lex_targets(args.targets, config) {
        for command in commands {
     		exec(command)?;
        }
      }
    */

    Ok(())
}
