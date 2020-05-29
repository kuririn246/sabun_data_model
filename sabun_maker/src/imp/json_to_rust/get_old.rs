use json5_parser::{JVal};
use crate::imp::json_to_rust::names::Names;
use std::collections::{HashSet};
use crate::error::Result;
use crate::imp::json_to_rust::json_name::{json_name, NameType};
use crate::structs::value_type::ValueType;

pub fn get_old(array : &[JVal], names : &Names) -> Result<HashSet<String>>{
    let mut result : HashSet<String> = HashSet::new();

    for item in array{
        match item{
            JVal::String(s, span) =>{
                match json_name(s){
                    Some(NameType::Name(name, ValueType::Normal)) =>{
                        result.insert(name);
                    },
                    _ =>{
                        Err(format!("{} {} is not a valid simple name {}",span.line_str(), s, names))?;
                    }
                }
            },
            _ =>{
                let span = item.span();
                Err(format!(r#"{} {} old must be strings {}"#, span.line_str(), span.slice(), names))?
            }
        }
    }
    return Ok(result);
}