use json5_parser::{JVal, Span};
use super::super::names::Names;
use super::get_default::get_default;
use crate::error::Result;
use std::collections::BTreeMap;
use crate::imp::json_to_rust::list::get_redef::get_redef;
use crate::structs::rust_object::RustObject;

pub enum ListAttribute{
    Default(RustObject),
    AutoID(Option<u64>),
    Reffered,
    Redef(BTreeMap<String, String>),
}


pub fn list_attribute(array : &Vec<JVal>, span : &Span, names : &Names) -> Result<ListAttribute>{
    let error_message = "List's array must be AutoID, Reffered or Default";

    if array.len() == 0{
        Err(format!("{} {} {} {}", span.line_str(), span.slice(), error_message, names))?
    }
    return match &array[0]{
        JVal::String(s, _) =>{
            match s.as_str(){
                "AutoID" =>{
                    if array.len() == 1 { Ok(ListAttribute::AutoID(None)) }
                    else if array.len() == 2{
                        match &array[1]{
                            JVal::Double(d, _) =>{
                                if *d as u64 as f64 == *d{
                                    Ok(ListAttribute::AutoID(Some(*d as u64)))
                                } else{
                                    Err(format!("{} {} AutoID's ID must be an integer {}", span.line_str(), span.slice(), names))?
                                }
                            },
                            _ =>{
                                Err(format!("{} {} AutoID's ID must be an integer {}", span.line_str(), span.slice(), names))?
                            }
                        }
                    } else {
                        Err(format!("{} {} <- [\"AutoID\"] is valid {}", span.line_str(), span.slice(), names))?
                    }
                },
                "Default" =>{
                    let def = get_default(&array[1..], span, names)?;
                    Ok(ListAttribute::Default(def))
                },
                "Reffered" =>{
                    if array.len() == 1 { Ok(ListAttribute::Reffered) }
                    else{ Err(format!("{} {} [\"Reffered\"] is valid {}", span.line_str(), span.slice(), names))? }
                },
                "Redef" =>{
                    let redef = get_redef(&array[1..], span, names)?;
                    Ok(ListAttribute::Redef(redef))
                },
                _ =>{
                    Err(format!("{} {} {} {}", span.line_str(), span.slice(), error_message, names))?
                }
            }
        },
        _ =>{
            Err(format!("{} {} {} {}", span.line_str(), span.slice(), error_message, names))?
        }
    };
}