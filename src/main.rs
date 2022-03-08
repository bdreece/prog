use std::io::Result;
use std::process::{Command, Stdio};
use std::path::PathBuf;

use clap::Parser;

mod alias;
mod cli;
mod command;
mod config;
mod invoc;
mod prelude;

use crate::cli::{Args, Format, Template};
use crate::command::Commands;
use crate::config::Config;
use crate::alias::Aliases;
use crate::invoc::Invocs;
use crate::prelude::*;

fn main() -> Result<()> {
    let args = Args::parse();

    let path = args.path.unwrap_or(PathBuf::from("."));
    let format = args.format.unwrap_or(Format::Yaml);
    let template = args.template.unwrap_or(Template::Bare);

    // Parse config data into memory
    let config = match args.generate {
		true => {
            if args.verbose > 0 {
            	println!("Generating config file");
            }
            let c = Config::new(&path, &format, template).unwrap();
            c.dump(&format).unwrap();
            c
        },
        false => {
            if args.verbose > 0 {
            	println!("Found config file");
            }
            Config::try_parse_from(&path).unwrap()
        }
    };

    // Convert and exit
    if args.convert {
        if args.verbose > 0 {
        	println!("Converting and dumping config file");
        }
    	config.dump(&format).unwrap();
        return Ok(());
    }

    // Parse config data per syntax
    let aliases = Aliases::try_parse_from(config).unwrap();
   	if args.verbose > 0 {
    	println!("Successfully parsed aliases from config file");
    }

    // Parse input script into memory
    let mut invocs: Invocs = vec![];
    if let Some(invoc_str) = &args.input {
        invocs = Invocs::try_parse_from(&String::from(invoc_str)).unwrap();
    } else if let Some(invoc_path) = &args.script {
    	invocs = Invocs::try_parse_from(invoc_path).unwrap();	
    }

    if args.verbose > 0 {
    	println!("Successfully parsed invocations from script");
    }

    // Match invocations to aliases and generate list of commands
    let mut commands: Commands = vec![];
    for invoc in &invocs {
    	commands.extend(Commands::try_parse_from((&mut invoc.clone(), &mut aliases.clone())).unwrap());
    }

    if args.verbose > 0 {
    	println!("Successfully parsed commands from aliases and invocations");
    }
	/*
	println!("Aliases: {:#?}", aliases);
    println!("Invocs: {:#?}", invocs);
    println!("Commands: {:#?}", commands);
	*/
	for command in commands {
    	let mut argv = command.split(" ");
        let program = argv.next().unwrap();
        if args.verbose > 0 {
        	println!("Executing command: {}", program);
        }
        Command::new(program)
        	.args(argv)
            .stdout(Stdio::inherit())
            .output()
            .expect("Failed to execute command");
    }

    Ok(())
}
