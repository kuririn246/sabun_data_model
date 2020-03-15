use json5_parser::{JVal, Span};
use crate::json_to_rust::names::Names;
use std::collections::{BTreeMap, HashMap};
use crate::error::Result;
use crate::json_to_rust::list::rename_map::{rename_map, RenameMapError};

pub fn get_renamed(array : &[JVal], span : &Span, names : &Names) -> Result<HashMap<String,String>>{
    let mut vec : Vec<(String, String)> = vec![];

    for item in array{
        match item{
            JVal::String(s, span) =>{
                let split : Vec<&str> = s.split("->").collect();
                if split.len() != 2{
                    Err(format!(r#"{} {} "oldName->currentName" is valid {}"#, span.line_col_str(), span.slice(), names))?
                }
                vec.push((split[0].to_string(), split[1].to_string()));
            },
            _ =>{
                let span = item.span();
                Err(format!(r#"{} {} "oldName->currentName" is valid {}"#, span.line_col_str(), span.slice(), names))?
            }
        }
    }
    match rename_map(vec){
        Ok(map) => Ok(map),
        Err(e) =>{
            match e{
                RenameMapError::DuplicatedOld(old) =>{
                    Err(format!(r#"{} {} is duplicated {}"#, span.line_col_str(), old, names))?
                },
                RenameMapError::CircularReference(s) =>{
                    Err(format!(r#"{} {} is circulated {}"#, span.line_col_str(), s, names))?
                }
            }
        }
    }

}