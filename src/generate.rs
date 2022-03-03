use std::collections::HashMap;
use std::io::Result;
use std::path::PathBuf;
use std::string::String;

use clap::ArgEnum;
use serde_any;

use crate::parse::ConfigValue;

#[derive(ArgEnum, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Format {
    Json,
    Ron,
    Toml,
    Yaml,
}

#[derive(ArgEnum, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Template {
    Bare,
    Cmake,
    Cargo,
    Go,
    Node,
    Make,
    Python,
}

pub fn generate_config_path(path: &PathBuf, format: &Format) -> PathBuf {
	let path = path.as_path();
    let config_path = match format {
        Format::Json => path.join("prog.json"),
        Format::Ron  => path.join("prog.ron"),
        Format::Toml => path.join("prog.toml"),
        Format::Yaml => path.join("prog.yml"),
    };
    config_path
}

pub fn generate_config(path: &PathBuf, format: &Format, template: Template) -> Result<()> {
	let config_path = generate_config_path(path, &format);
	let template_data: HashMap<String, ConfigValue> = match template {
        Template::Bare   => HashMap::new(),
    	Template::Cmake  => serde_any::from_file("templates/cmake.yml").unwrap(), 
    	Template::Cargo  => serde_any::from_file("templates/cargo.yml").unwrap(), 
    	Template::Go     => serde_any::from_file("templates/go.yml").unwrap(), 
    	Template::Node   => serde_any::from_file("templates/node.yml").unwrap(), 
    	Template::Make   => serde_any::from_file("templates/make.yml").unwrap(), 
    	Template::Python => serde_any::from_file("templates/python.yml").unwrap(),
    };

	serde_any::to_file_pretty(config_path, &template_data).unwrap();
    Ok(())
}
