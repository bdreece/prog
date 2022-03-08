use crate::prelude::*;
use crate::alias::{AliasKey, Aliases, Alias, AliasValue};
use crate::invoc::{Invoc, InvocType};

pub type Commands = Vec<String>;

impl TryParseFrom<(&mut Invoc, &mut Aliases)> for Commands {
	type Error = String;
    fn try_parse_from(value: (&mut Invoc, &mut Aliases)) -> Result<Self, Self::Error>
    where Self: Sized {
        // TODO: Find and resolve aliased command from invocation
        let invoc = value.0;
        let aliases = value.1;
        let mut alias: Alias = aliases.iter()
            				   .filter(|&a| {
                        			invoc.alias_key == match a.key.clone() {
        								AliasKey::Declaration(s) => s,
            							AliasKey::Function(s) => s
                    	            }
                                }).last().expect("Alias not found!").clone();

        return match &mut invoc.invoc_type {
            InvocType::Declarative => {
            	match &alias.value {
                	AliasValue::Command(s) => Ok(vec![s.clone()]),
                    AliasValue::List(list) => Ok(list.clone()),
                    AliasValue::Map(_) => Err(
                        String::from("Cannot invoke map alias declaratively")
                    ),
                }
            },
            InvocType::Imperative(args) => {
                alias.process(&args);
                match &alias.value {
                	AliasValue::Command(s) => Ok(vec![s.clone()]),
                    AliasValue::List(list) => Ok(list.clone()),
                    AliasValue::Map(_) => Err(
                        String::from("Cannot invoke map alias imperatively")
                    ),
                }
            },
            InvocType::Scoped(sub_invocs) => {
            	match &mut alias.value {
                	AliasValue::Map(aliases) => 
                        Ok(sub_invocs.iter_mut()
                        	         .map(|sub_invoc| Commands::try_parse_from((sub_invoc, aliases)).unwrap())
                                  	 .fold(vec![], |mut a, b| {a.extend(b); a})),
                    _ => Err(
                        String::from("Only map aliases can be invoked with a scope")
                    ),
                }
            },
            InvocType::Indexical(indices) => {
            	match &alias.value {
                    AliasValue::List(list) => Ok(list.iter()
                        						 	 .enumerate()
                                                  	 .filter_map(|elem| {
                                                         if indices.contains(&elem.0.into()) {
															return Some(elem.1.clone());
                                                         }
                                                         None
                                                     }).collect()),
                	_ => return Err (
                        String::from("Only list aliases be indexed")
                    ),
                }
            },
			InvocType::Selective(invoc) => {
            	match &mut alias.value {
                	AliasValue::Map(aliases) => {
                    	Ok(Commands::try_parse_from((invoc, aliases)).unwrap())	
                    },
                    _ => Err(
                    	String::from("Only map aliases can be invoked selectively")
                    ),
                }
            }
        };
    }
}
