use json5_parser::{JVal, Span};
use super::super::names::Names;
use super::get_default::get_default;
use crate::error::Result;
use std::collections::{ HashSet};
use crate::imp::json_to_rust::get_old::get_old;
use crate::structs::root_object::ListDefObj;

pub enum ListAttribute{
    Default(ListDefObj),
    Old(HashSet<String>),
    Compatible(HashSet<String>),
}


pub fn list_attribute(array : &Vec<JVal>, span : &Span, names : &Names) -> Result<ListAttribute>{
    let error_message = "List's array must be Default or Old";

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
                    let compatible = get_old(&array[1..], names)?;
                    Ok(ListAttribute::Compatible(compatible))
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