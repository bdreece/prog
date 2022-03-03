use std::collections::HashMap;
use std::option::Option;
use std::string::String;
use std::vec::Vec;

use crate::parse::ConfigValue;

pub fn _lex_targets(targets: String, _config: HashMap<String, ConfigValue>) -> Option<Vec<String>> {
	if targets.is_empty() {
    	return None;
    }

    // TODO: Implement weird DFS/BFS hybrid algo
	for target in targets.split(";") {
        // Trim whitespace
        let target = target.trim();
		if target.contains("{") && target.contains("}") {

        }
    }

    Some(Vec::new())
}
