use std::collections::HashMap;
use std::fs::remove_file;
use std::io::Result;
use std::path::PathBuf;

use serde_any;

use crate::parse::{ConfigValue, find_config};
use crate::generate::{Format, generate_config_path};

pub fn convert_config(path: &PathBuf, format: &Format) -> Result<()> {
    let input_file = find_config(path).unwrap();
	let output_file = generate_config_path(path, format);
	let buffer: HashMap<String, ConfigValue> = serde_any::from_file(input_file.clone()).unwrap();
    remove_file(input_file)?;
    serde_any::to_file_pretty(output_file, &buffer).unwrap();
    Ok(())
}
