use json5_parser::{JVal, Span};
use super::super::names::Names;
use super::get_default::get_default;
use crate::error::Result;
use std::collections::{ HashSet};
use crate::imp::json_to_rust::get_old::get_old;
use crate::structs::root_object::ListDefObj;
use crate::imp::json_to_rust::get_compatible::get_compatible;

pub enum ListAttribute{
    Default(ListDefObj),
    Old(HashSet<String>),
    Compatible(HashSet<String>),
    NextID(u64),
}


pub fn list_attribute(array : &Vec<JVal>, span : &Span, names : &Names) -> Result<ListAttribute>{
    let error_message = "List's array must be Default, Old, Compatible or NextID";

    if array.len() == 0{
        Err(format!("{} {} {} {}", span.line_str(), span.slice(), error_message, names))?
    }
    return match &array[0]{
        JVal::String(s, _) =>{
            match s.as_str(){
                "Default" =>{
                    let def = get_default(&array[1..], span, names)?;
                    Ok(ListAttribute::Default(def))
                },
                "Old" =>{
                    let old = get_old(&array[1..], names)?;
                    Ok(ListAttribute::Old(old))
                },
                "Compatible" =>{
                    let compatible = get_compatible(&array[1..], names)?;
                    Ok(ListAttribute::Compatible(compatible))
                },
                "NextID" =>{
                    if array.len() == 2{
                        match array[1]{
                            JVal::Double(n, _) =>{
                                return Ok(ListAttribute::NextID(n as u64))
                            }
                            _ =>{}
                        }
                    }
                    Err(format!(r#"{} {} NextID must be ["NextID", num] {}"#, span.line_str(), span.slice(), names))?
                }
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