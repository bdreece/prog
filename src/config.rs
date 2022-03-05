use std::collections::HashMap;
use std::io::ErrorKind;
use std::path::PathBuf;

use serde_any;

use crate::prelude::TryParseFrom;
use crate::cli::{Format, Template};
use crate::alias::AliasValue;

pub struct Config {
    pub path: Box<PathBuf>,
    pub entry: Box<PathBuf>,
    pub data: HashMap<String, AliasValue>,
}

impl TryParseFrom<&PathBuf> for Config {
	type Error = std::io::Error;
    fn try_parse_from(value: &PathBuf) -> Result<Self, Self::Error> {	
    	let path = value.as_path();
        let mut config_path: Option<PathBuf> = None;
    	// Read entries in path
    	for entry in path.read_dir()? {
        	let entry = entry.unwrap().path();
        	let entry_path = entry.as_path();
        	// If entry has 'prog' prefix
        	if let Some(prefix) = entry_path.file_stem() {
            	if prefix.to_str().unwrap() == "prog" {
                	// Found config file
                	config_path = Some(entry.clone());
                    break;
            	}
        	}
    	}

        match config_path {
        	None => Err(Self::Error::new(ErrorKind::NotFound, String::from("No config file found"))),
			Some(c) => Ok(
                Config {
            		path: Box::new(value.clone()),
					entry: Box::new(c.clone()),
           			data: serde_any::from_file(c).unwrap(),
            	}
            )
        }
    }
}

fn new_filepath(path: &PathBuf, format: &Format) -> PathBuf {
	return match format {
    	Format::Json => path.as_path()
                            .join("prog.json"),
    	Format::Ron =>  path.as_path()
                            .join("prog.ron"),
    	Format::Toml => path.as_path()
                            .join("prog.toml"),
    	Format::Yaml => path.as_path()
                            .join("prog.yml"),
	};
}

impl Config {
	pub fn dump(&self, format: &Format) -> Result<(), serde_any::Error> {
    	let output_file = new_filepath(self.path.as_ref(), format);
    	serde_any::to_file_pretty(output_file, &self.data)?;
    	Ok(())
	}


	pub fn new(path: &PathBuf, format: &Format, template: Template) -> Result<Config, serde_any::Error> {
    	let entry = new_filepath(&path, &format);
        let data: HashMap<String, AliasValue> = match template {
        	Template::Bare => HashMap::new(),
        	Template::Cmake => serde_any::from_file("templates/cmake.yml")?,
        	Template::Cargo => serde_any::from_file("templates/cargo.yml")?,
        	Template::Go => serde_any::from_file("templates/go.yml")?,
        	Template::Node => serde_any::from_file("templates/node.yml")?,
        	Template::Make => serde_any::from_file("templates/make.yml")?,
        	Template::Python => serde_any::from_file("templates/python.yml")?,
    	};

    	Ok(Config {
        	path: Box::new(path.clone()),
			entry: Box::new(entry),
            data
        })
	}
}
