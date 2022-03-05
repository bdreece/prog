use std::vec::Vec;

use lazy_static::lazy_static;
use regex::Regex;

use crate::prelude::TryParseFrom;

#[derive(Debug)]
pub enum InvocType {
    Declarative,
    Imperative(Vec<String>),
    Scoped(Vec<Invoc>),
    Indexical(Vec<usize>),
    Selective(Box<Invoc>),
}

#[derive(Debug)]
pub struct Invoc {
	alias_key: String,
    invoc_type: InvocType,
}

pub type Invocs = Vec<Invoc>;

impl TryParseFrom<&str> for Invoc {
    type Error = regex::Error;
	fn try_parse_from(value: &str) -> Result<Self, Self::Error> {
    	lazy_static! {
    	    static ref DECL_RE: Regex = Regex::new(r"^([[:alnum:]]+)$").unwrap();
    	    static ref IMP_RE: Regex = Regex::new(r"^([[:alpha:]]+)\((.*)\)$").unwrap();
    	    static ref SCOPE_RE: Regex = Regex::new(r"^([[:alnum:]]+)\{(.*)\}$").unwrap();
    	    static ref INDEX_RE: Regex = Regex::new(r"^([[:alnum:]]+)\[(.*)\]$").unwrap();
    	    static ref SELECT_RE: Regex = Regex::new(r"^([[:alnum:]]+)\.(.*)$").unwrap();
    	    static ref ARG_RE: Regex = Regex::new(r"\s?([^,]+)").unwrap();
            
            static ref MUL_ERR: regex::Error = regex::Error::Syntax(
                String::from("Multiple invocations in statement")
            );
            static ref INV_ERR: regex::Error = regex::Error::Syntax(
            	String::from("Invalid invocation syntax")
            );
    	}
		
    	let mut invoc: Option<Invoc> = None;
		
		if let Some(captures) = DECL_RE.captures(value) {
    	    invoc = Some(Invoc {
    	        alias_key: String::from(captures.get(1)
    	                       				    .unwrap()
    	                                        .as_str()),
    	        invoc_type: InvocType::Declarative,
    	    });
    	} else if let Some(captures) = IMP_RE.captures(value) {
    	    let mut args: Vec<String> = vec![];

    	    for arg in ARG_RE.captures_iter(&captures[2]) {
    	        args.push(String::from(arg.get(1)
                        				  .unwrap()
                                          .as_str()));
    	    }

    		invoc = Some(Invoc {
    	    	alias_key: String::from(captures.get(1)
    	                       					.unwrap()
    	                                        .as_str()),
    	        invoc_type: InvocType::Imperative(args),
    	    });
    	} else if let Some(captures) = SCOPE_RE.captures(value) {
            let mut args: Vec<Invoc> = vec![];

    	    for arg in ARG_RE.captures_iter(&captures[2]) {
    	        let arg_invoc = Invoc::try_parse_from(&arg[1])?;
    	        args.push(arg_invoc);
    	    }

    		invoc = Some(Invoc {
    	    	alias_key: String::from(captures.get(1)
    	                       					.unwrap()
    	                                        .as_str()),
    	        invoc_type: InvocType::Scoped(args),
    	    });
    	} else if let Some(captures) = INDEX_RE.captures(value) {
            let mut indices: Vec<usize> = vec![];

    	    for i in ARG_RE.captures_iter(&captures[2]) {
    	        indices.push(i[1].parse::<usize>().unwrap());
    	    }

    	    invoc = Some(Invoc {
    	        alias_key: String::from(captures.get(1)
    	                       					.unwrap()
    	                                        .as_str()),
    	        invoc_type: InvocType::Indexical(indices)
    	    });
    	} else if let Some(captures) = SELECT_RE.captures(value) {
            let arg_invoc = Invoc::try_parse_from(&captures[2])?;
    	    invoc = Some(Invoc {
    	        alias_key: String::from(captures.get(1)
    	                       					.unwrap()
    	                                        .as_str()),
    	        invoc_type: InvocType::Selective(Box::new(arg_invoc)),
    	    });
    	}

    	if invoc.is_none() {
        	return Err(INV_ERR.clone());
        }

    	Ok(invoc.unwrap())
    }
}

impl TryParseFrom<&str> for Invocs {
    type Error = regex::Error;
	fn try_parse_from(invocs: &str) -> Result<Invocs, Self::Error> {
    	lazy_static! {
        	static ref DELIM_RE: Regex = Regex::new(r";\s?").unwrap();
    	}

		let mut invoc_list: Invocs = vec![];

    	for stmt in DELIM_RE.split(invocs) {
        	let invoc = Invoc::try_parse_from(stmt).unwrap();
    		invoc_list.push(invoc);
        }

    	Ok(invoc_list)
	}
}
