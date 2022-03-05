use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::string::String;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_any;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ConfigValue {
    Command(String),
    List(Vec<String>),
    Map(HashMap<String, ConfigValue>),
}

#[derive(Debug)]
pub enum ConfigKeyType {
	Declarative,
    Functional(usize),
}

#[derive(Debug)]
pub struct ConfigEntry {
	key: String,
    key_type: ConfigKeyType,
    value: ConfigValue,
}

#[derive(Debug)]
pub enum AliasType {
    Declarative,
    Imperative(Vec<String>),
    Scoped(Vec<(String, AliasType)>),
    Indexical(Vec<usize>),
    Selective((String, Box<AliasType>)),
}

pub fn parse_target(target: &str) -> Result<(String, AliasType), regex::Error> {
    lazy_static! {
        static ref DECL_RE: Regex = Regex::new(r"^([[:alnum:]]+)$").unwrap();
        static ref IMP_RE: Regex = Regex::new(r"^([[:alpha:]]+)\((.*)\)$").unwrap();
        static ref SCOPE_RE: Regex = Regex::new(r"^([[:alnum:]]+)\{(.*)\}$").unwrap();
        static ref INDEX_RE: Regex = Regex::new(r"^([[:alnum:]]+)\[(.*)\]$").unwrap();
        static ref SELECT_RE: Regex = Regex::new(r"^([[:alnum:]]+)\.(.*)$").unwrap();
        static ref ARG_RE: Regex = Regex::new(r"\s?([^,]+)").unwrap();
    }

    for capture in DECL_RE.captures_iter(target) {
        return Ok((capture[1].into(), AliasType::Declarative));
    }

    for capture in IMP_RE.captures_iter(target) {
        let mut args: Vec<String> = vec![];
        for arg in ARG_RE.captures_iter(&capture[2]) {
            args.push(arg[1].into());
        }
        return Ok((capture[1].into(), AliasType::Imperative(args)));
    }

    for capture in SCOPE_RE.captures_iter(target) {
        let mut args: Vec<(String, AliasType)> = vec![];
        for arg in ARG_RE.captures_iter(&capture[2]) {
            let (alias, alias_type) = parse_target(&arg[1])?;
            args.push((alias, alias_type));
        }
        return Ok((capture[1].into(), AliasType::Scoped(args)));
    }

    for capture in INDEX_RE.captures_iter(target) {
        let mut indices: Vec<usize> = vec![];
        for i in ARG_RE.captures_iter(&capture[2]) {
            indices.push(i[1].parse::<usize>().unwrap());
        }
        return Ok((capture[1].into(), AliasType::Indexical(indices)));
    }

    for capture in SELECT_RE.captures_iter(target) {
        let (alias, alias_type) = parse_target(&capture[2])?;
        return Ok((capture[1].into(), AliasType::Selective((alias, Box::new(alias_type)))));
    }

    Ok((String::new(), AliasType::Declarative))
}

pub fn parse_targets(targets: &str) -> std::io::Result<Vec<(String, AliasType)>> {
    lazy_static! {
        static ref DELIM_RE: Regex = Regex::new(r";\s?").unwrap();
    }

	let mut target_list: Vec<(String, AliasType)> = vec![];

    for target in DELIM_RE.split(targets) {
        let (alias, alias_type) = parse_target(target).unwrap();
        if alias != "" {
            target_list.push((alias, alias_type));
        }
    }

    Ok(target_list)
}
pub fn find_config(path: &PathBuf) -> std::io::Result<PathBuf> {
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

pub fn parse_config(path: &PathBuf) -> std::io::Result<Vec<ConfigEntry>> {
	lazy_static! {
		static ref FUNC_RE: Regex = Regex::new(r"^([[:alpha:]]+)\(([0-9]+)\)$").unwrap();
	}

    let file = find_config(&path)?;
    let raw_config: HashMap<String, ConfigValue> = serde_any::from_file(file).unwrap();
    let mut parsed_config: Vec<ConfigEntry> = vec![];
    for (key, value) in raw_config.iter() {
        let mut config_key_type: Option<ConfigKeyType> = None;
        let mut config_key = key.clone();
    	for capture in FUNC_RE.captures_iter(key.as_str()) {
        	config_key = capture[1].into();	
            config_key_type = Some(
                ConfigKeyType::Functional(capture[2].parse::<usize>().unwrap())
            );
        }

        if config_key_type.is_none() {
        	config_key_type = Some(ConfigKeyType::Declarative);
        }

        parsed_config.push(ConfigEntry{
            key: config_key,
			key_type: config_key_type.unwrap(),
            value: value.clone()
        });
    }
    Ok(parsed_config)
}
