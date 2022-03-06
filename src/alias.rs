use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::prelude::*;
use crate::config::{Config, ConfigValue};

#[derive(Clone, Debug)]
pub enum AliasValue {
    Command(String),
    List(Vec<String>),
    Map(Aliases),
}

#[derive(Clone, Debug)]
pub enum AliasKey {
	Declaration(String),
    Function(String),
}

#[derive(Clone, Debug)]
pub struct Alias {
    pub key: AliasKey,
    pub value: AliasValue,
}

pub type Aliases = Vec<Alias>;

impl Alias {
	fn replace_arg(cmd: &mut String, i: usize, arg: &String) {
        lazy_static! {
        	static ref VAR_RE: Regex = Regex::new(r"(\$\d+)").unwrap();
        }

        *cmd = VAR_RE.replace(cmd, |caps: &Captures| {
        	return match caps.get(i) {
            	Some(_) => arg.clone(),
                None => cmd.clone(),
            } 
        }).to_string();
    }
}

impl Process for Alias {
	fn process(&mut self, args: &Vec<String>) {
    	match &self.key {
        	AliasKey::Declaration(_) => return,
            AliasKey::Function(_) => {
           		match &mut self.value {
           			AliasValue::Command(cmd) => {
           	    		for (i, arg) in args.iter().enumerate() {
                        	Alias::replace_arg(cmd, i, arg);
                    	}
           	    	},
           	    	AliasValue::List(list) => {
           	    		for cmd in list.iter_mut() {
                    		for (i, arg) in args.iter().enumerate() {
                        		Alias::replace_arg(cmd, i, arg);
                        	}
                    	}
           	    	},
           	    	AliasValue::Map(m) => {
           	    		for alias in m {
                        	alias.process(&args);
                        }
           	    	}
           		}
       		}
        }
    }
}

impl Process for Aliases {
	fn process(&mut self, args: &Vec<String>) {
    	for alias in self {
        	alias.process(&args);	
        }
    }
}

impl TryParseFrom<(String, ConfigValue)> for Alias {
    type Error = regex::Error;
	fn try_parse_from(value: (String, ConfigValue)) -> Result<Self, Self::Error> {
		lazy_static! {
			static ref FUNC_RE: Regex = Regex::new(r"^([[:alpha:]]+)\(\)$").unwrap();
		}

    	let mut key = AliasKey::Declaration(value.0.clone());

    	if let Some(captures) = FUNC_RE.captures(value.0.as_ref()) {
    	    key = AliasKey::Function(captures.get(1)
                    					      .unwrap()
                                              .as_str()
                                              .into());
    	}
		
        let new_value: AliasValue = match &value.1 {
        	ConfigValue::Command(c) => AliasValue::Command(c.clone()),
            ConfigValue::List(l) => AliasValue::List(l.clone()),
            ConfigValue::Map(m) => {
            	let mut aliases: Aliases = vec![];

                for (key, val) in m {
                	aliases.push(Alias::try_parse_from((key.clone(), val.clone())).unwrap());	
                }

                AliasValue::Map(aliases)
            },
        };

		Ok(Alias {
        	key,
            value: new_value,
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
