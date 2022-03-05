use std::io::Result;
use std::path::PathBuf;

use clap::Parser;

mod cli;
mod convert;
mod exec;
mod generate;
mod parse;
mod traverse;

use crate::cli::Args;
use crate::convert::convert_config;
use crate::generate::{generate_config, Format, Template};
use crate::parse::{parse_config, parse_targets, AliasType};

fn main() -> Result<()> {
    let args = Args::parse();

    let path = args.path.unwrap_or(PathBuf::from("."));
    let format = args.format.unwrap_or(Format::Yaml);
    let template = args.template.unwrap_or(Template::Bare);

    if args.generate {
        generate_config(&path, &format, template)?;
    }

    if args.convert {
        convert_config(&path, &format)?;
    }

    let mut targets: Vec<(String, AliasType)> = vec![];
    let config = parse_config(&path)?;
    if let Some(target_string) = &args.targets {
        targets = parse_targets(&target_string)?;
    }

    println!("Config: {:#?}", config);
    println!("");
    println!("Targets: {:#?}", targets);

    Ok(())
}
