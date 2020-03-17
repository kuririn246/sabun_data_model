use crate::rust_struct::{RustValue, Qv, ValueType};
use super::Names;
use json5_parser::JVal;
use crate::error::Result;
use crate::json_to_rust::json_array_to_rust::json_array_to_rust;


pub fn json_item_to_rust(name : &str, value_type : ValueType, v : &JVal, names : &Names) -> Result<RustValue> {
    let names = &names.append(name);
    match v {
        JVal::Bool(b, _) => {
            Ok(RustValue::Bool(Qv::Val(*b), value_type))
        },
        JVal::Double(f, _)=>{
            Ok(RustValue::Number(Qv::Val(*f), value_type))
        }
        JVal::String(s, _) => {
            let s = s.to_string();
            Ok(RustValue::String(Qv::Val(s), value_type))
        },
        JVal::Array(a, _) => {
            Ok(json_array_to_rust(a, value_type, v.span(), names)?)
        },
        JVal::Map(_map, _) => {
            Err(format!("An object can't have an object"))?
        },
        JVal::Null(_) =>{
             Err(format!(r#"{} null must be ["type", null] {}"#, v.line_col(), names))?
        },
    }
}


