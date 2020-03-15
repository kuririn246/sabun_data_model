use crate::rust_struct::{RustObject, ValueType, RefName};
use std::collections::BTreeMap;
use json5_parser::{JVal, Span};
use crate::json_to_rust::names::Names;

pub enum ListAttribute{
    Default(RustObject),
    AutoID,
    Ref(Vec<RefName>),
    Renamed(BTreeMap<String, String>),
    Reffered,
}


pub fn list_attribute(array : &Vec<JVal>, span : &Span, names : &Names) -> Result<ListAttribute>{
    let error_message = "List's array must be AutoID, Ref, Reffered, Renamed or Default";

    if array.len() == 0{
        Err(format!("{} {} {} {}", span.line_col_str(), span.slice(), error_message, names))?
    }
    return match &array[0]{
        JVal::String(s, _) =>{
            match s.as_str(){
                "AutoID" =>{ Ok(ListArrayItem::AutoID) },
                "Default" =>{ get_default(&array[1..], span, names) },
                "Ref" =>{ todo!() },
                "Reffered" =>{
                    if array.len() == 1 { Ok(ListAttribute::Reffered) }
                    else{ Err(format!("{} {} ["Reffered"] is valid {}", span.line_col_str(), span.slice(), error_message, names))? }
                },
                "ListID" =>{ todo!() },
                _ =>{
                    Err(format!("{} {} {} {}", span.line_col_str(), span.slice(), error_message, names))?
                }
            }
        },
        _ =>{
            Err(format!("{} {} {} {}", span.line_col_str(), span.slice(), error_message, names))?
        }
    };
}