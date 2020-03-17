use crate::rust_struct::{RustObject};
use json5_parser::{JVal, Span};
use crate::json_to_rust::names::Names;
use crate::json_to_rust::list::get_default::get_default;
use crate::error::Result;

pub enum ListAttribute{
    Default(RustObject),
    AutoID(Option<u64>),
    Reffered,
}


pub fn list_attribute(array : &Vec<JVal>, span : &Span, names : &Names) -> Result<ListAttribute>{
    let error_message = "List's array must be AutoID, Reffered or Default";

    if array.len() == 0{
        Err(format!("{} {} {} {}", span.line_col_str(), span.slice(), error_message, names))?
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
                                    Err(format!("{} {} AutoID's ID must be an integer {}", span.line_col_str(), span.slice(), names))?
                                }
                            },
                            _ =>{
                                Err(format!("{} {} AutoID's ID must be an integer {}", span.line_col_str(), span.slice(), names))?
                            }
                        }
                    } else {
                        Err(format!("{} {} <- [\"AutoID\"] is valid {}", span.line_col_str(), span.slice(), names))?
                    }
                },
                "Default" =>{
                    let def = get_default(&array[1..], span, names)?;
                    Ok(ListAttribute::Default(def))
                },
                "Reffered" =>{
                    if array.len() == 1 { Ok(ListAttribute::Reffered) }
                    else{ Err(format!("{} {} [\"Reffered\"] is valid {}", span.line_col_str(), span.slice(), names))? }
                },

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