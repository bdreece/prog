use std::io::Result;
use std::path::PathBuf;

use clap::Parser;

mod alias;
mod cli;
mod config;
mod invoc;
mod prelude;

use crate::cli::{Args, Format, Template};
use crate::config::Config;
use crate::alias::Aliases;
use crate::invoc::Invocs;
use crate::prelude::TryParseFrom;

fn main() -> Result<()> {
    let args = Args::parse();

    let path = args.path.unwrap_or(PathBuf::from("."));
    let format = args.format.unwrap_or(Format::Yaml);
    let template = args.template.unwrap_or(Template::Bare);

    let config = match args.generate {
		true => {
            let c = Config::new(&path, &format, template).unwrap();
            c.dump(&format).unwrap();
            c
        },
        false => Config::try_parse_from(&path).unwrap(),
    };

    if args.convert {
    	config.dump(&format).unwrap();
    }

    let aliases = Aliases::try_parse_from(config).unwrap();
    let mut invocs: Invocs = vec![];
    
    if let Some(invoc_str) = &args.input {
        invocs = Invocs::try_parse_from(&String::from(invoc_str)).unwrap();
    } else if let Some(invoc_path) = &args.script {
    	invocs = Invocs::try_parse_from(invoc_path).unwrap();	
    }

    println!("Aliases: {:#?}", aliases);
    println!("");
    println!("Invocations: {:#?}", invocs);

    Ok(())
}
