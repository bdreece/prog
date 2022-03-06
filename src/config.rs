use std::collections::HashMap;
use std::io::ErrorKind;
use std::path::PathBuf;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_any;

use crate::prelude::TryParseFrom;
use crate::cli::{Format, Template};

static TEMPLATE_CMAKE_STR: &str = "{\"build\":\"cmake --build ./build\",\"test\":\"ctest -V --test-dir=./build/tests\",\"configure\":{\"release\":[\"mkdir -p build\",\"cmake -DCMAKE_BUILD_TYPE=Release -B ./build .\"],\"debug\":[\"mkdir -p build\",\"cmake -DCMAKE_BUILD_TYPE=Debug -B ./build .\"]},\"push\":[\"git add .\",\"git commit\",\"git push\"]}";

static TEMPLATE_CARGO_STR: &str = "{\"build\":{\"debug\":\"cargo build\",\"release\":\"cargo build --release\"},\"run\":\"cargo run\",\"test\":\"cargo test\",\"push\":[\"git add .\",\"git commit\",\"git push\"]}";

static TEMPLATE_GO_STR: &str = "{\"run\":\"go run .\",\"test\":\"go test\",\"build\":\"go build\",\"push\":[\"git add .\",\"git commit\",\"git push\"]}";

static TEMPLATE_MAKE_STR: &str = "{\"build\":\"make all\",\"push\":[\"git add .\",\"git commit\",\"git push\"],\"run(1)\":\"./$1\"}";

static TEMPLATE_NODE_STR: &str = "{\"run\":\"npm start\",\"build\":\"npm run build\",\"push\":[\"git add .\",\"git commit\",\"git push\"]}";

static TEMPLATE_PYTHON_STR: &str = "{\"push\":[\"git add .\",\"git commit\",\"git push\"],\"run(1)\":\"python $1.py\"}";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ConfigValue {
	Command(String),
    List(Vec<String>),
    Map(HashMap<String, ConfigValue>),
}

pub struct Config {
    pub path: Box<PathBuf>,
    pub entry: Box<PathBuf>,
    pub data: HashMap<String, ConfigValue>,
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
    	lazy_static! {
        	static ref TEMPLATE_BARE: HashMap<String, ConfigValue> = HashMap::new();
            static ref TEMPLATE_CMAKE: HashMap<String, ConfigValue> = 
                serde_any::from_str(TEMPLATE_CMAKE_STR, serde_any::Format::Json).unwrap();
            static ref TEMPLATE_CARGO: HashMap<String, ConfigValue> = 
                serde_any::from_str(TEMPLATE_CARGO_STR, serde_any::Format::Json).unwrap();
            static ref TEMPLATE_GO: HashMap<String, ConfigValue> = 
                serde_any::from_str(TEMPLATE_GO_STR, serde_any::Format::Json).unwrap();
            static ref TEMPLATE_MAKE: HashMap<String, ConfigValue> = 
                serde_any::from_str(TEMPLATE_MAKE_STR, serde_any::Format::Json).unwrap();
            static ref TEMPLATE_NODE: HashMap<String, ConfigValue> =
                serde_any::from_str(TEMPLATE_NODE_STR, serde_any::Format::Json).unwrap();
            static ref TEMPLATE_PYTHON: HashMap<String, ConfigValue> =
                serde_any::from_str(TEMPLATE_PYTHON_STR, serde_any::Format::Json).unwrap();
        }

        let entry = new_filepath(&path, &format);
        let data: HashMap<String, ConfigValue> = match template {
        	Template::Bare => TEMPLATE_BARE.clone(),
        	Template::Cmake => TEMPLATE_CMAKE.clone(),
        	Template::Cargo => TEMPLATE_CARGO.clone(),
        	Template::Go => TEMPLATE_GO.clone(),
        	Template::Node => TEMPLATE_NODE.clone(),
        	Template::Make => TEMPLATE_MAKE.clone(),
        	Template::Python => TEMPLATE_PYTHON.clone(),
    	};

    	Ok(Config {
        	path: Box::new(path.clone()),
			entry: Box::new(entry),
            data
        })
	}
}
