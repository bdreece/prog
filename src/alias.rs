use std::collections::HashMap;
use std::string::String;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::prelude::TryParseFrom;
use crate::config::Config;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum AliasValue {
    Command(String),
    List(Vec<String>),
    Map(HashMap<String, AliasValue>),
}

#[derive(Clone, Debug)]
pub enum AliasKey {
	Declaration(String),
    Function((String, usize)),
}

#[derive(Debug)]
pub struct Alias {
    key: AliasKey,
    value: AliasValue,
}

pub type Aliases = Vec<Alias>;

impl Alias {
	pub fn key(&self) -> String {
    	match self.key.clone() {
        	AliasKey::Declaration(s) => s,
            AliasKey::Function((s, _)) => s,
        }
    }

    pub fn value(&self) -> &AliasValue {
    	&self.value
    }
}

impl TryParseFrom<(String, AliasValue)> for Alias {
    type Error = regex::Error;
	fn try_parse_from(value: (String, AliasValue)) -> Result<Self, Self::Error> {
		lazy_static! {
			static ref FUNC_RE: Regex = Regex::new(r"^([[:alpha:]]+)\((\d+)\)$").unwrap();
		}

    	let mut key = AliasKey::Declaration(value.0.clone());

    	if let Some(captures) = FUNC_RE.captures(value.0.as_ref()) {
    	    key = AliasKey::Function((captures.get(1)
                    					      .unwrap()
                                              .as_str()
                                              .into(),
                                      captures.get(2)
                    						  .unwrap()
                                              .as_str()
                                              .parse::<usize>()
                                              .unwrap()));
    	}

        Ok(Alias {
        	key,
            value: value.1
        })
    }
}

impl TryParseFrom<Config> for Aliases {
    type Error = regex::Error;
	fn try_parse_from(value: Config) -> Result<Self, Self::Error> {
    	let mut aliases: Aliases = vec![];
    	for (key, value) in value.data.iter() {
			let alias = Alias::try_parse_from((key.clone(), value.clone()))?;
    	    aliases.push(alias);
    	}
    	Ok(aliases)
    }
}
