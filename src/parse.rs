use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;
use std::string::String;

use serde::{Deserialize, Serialize};
use serde_any;

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ConfigValue {
    Command(String),
	List(Vec<String>),
    Map(HashMap<String, ConfigValue>),
}

pub fn find_config(path: &PathBuf) -> Result<PathBuf> {
    let path = path.as_path();
    // Read entries in path
    for entry in path.read_dir()? {
        let entry = entry.unwrap().path();
        let entry_path = entry.as_path();
        // If entry has prefix
        if let Some(prefix) = entry_path.file_stem() {
            if prefix.to_str().unwrap() == "prog" {
                // Found config file
                return Ok(entry);
            }
        }
    }
	Err(Error::new(ErrorKind::NotFound, "Config file not found"))
}

pub fn parse_config(path: &PathBuf) -> Result<HashMap<String, ConfigValue>> {
	let file = find_config(&path)?;
    let config: HashMap<String, ConfigValue> = serde_any::from_file(file).unwrap();
    Ok(config)
}
