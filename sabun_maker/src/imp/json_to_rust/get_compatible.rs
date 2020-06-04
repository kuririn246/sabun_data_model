use json5_parser::{JVal};
use crate::imp::json_to_rust::names::Names;
use std::collections::{HashSet};
use crate::error::Result;
use crate::imp::json_to_rust::json_name::{dot_chained_name};

pub fn get_compatible(array : &[JVal], names : &Names) -> Result<HashSet<String>>{
    let mut result : HashSet<String> = HashSet::with_capacity(array.len());

    for item in array{
        match item{
            JVal::String(s, span) =>{
                match dot_chained_name(s){
                    Some(s) =>{
                        result.insert(s.to_string());
                    },
                    _ =>{
                        Err(format!("{} {} is not a valid dot-chained name {}",span.line_str(), s, names))?;
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