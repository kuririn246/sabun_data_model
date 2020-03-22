use crate::imp::json_to_rust::names::Names;
use json5_parser::JVal;
use crate::imp::json_to_rust::json_array_to_rust::get_array;
use crate::rust_struct::{ArrayType, Qv, RustValue};
use crate::error::Result;

pub fn get_include(v : &JVal, names : &Names) -> Result<Vec<String>>{
    let names = &names.append("Include");
    let error_message = "Include must be an array with file-stems";
    let array = v.as_array().ok_or_else(|| format!("{} {} {}", v.line_str(), error_message, names))?;
    let array = get_array(array, &ArrayType::String, names)?;
    match array{
        Qv::Val(val) =>{
            let mut incl : Vec<String> = vec![];
            for item in &val.vec{
                match item{
                    RustValue::String(s, _) =>{
                        match s{
                            Qv::Val(s) =>{ incl.push(s.to_string()); },
                            _ => Err(format!("{} {} {}", v.line_str(), error_message, names))?,
                        }
                    },
                    _ => Err(format!("{} {} {}", v.line_str(), error_message, names))?,
                }
            }
            return Ok(incl);
        },
        _ =>{  Err(format!("{} {} {}", v.line_str(), error_message, names))? }
    }
}