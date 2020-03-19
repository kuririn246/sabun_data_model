use crate::rust_struct::{RustValue, Qv, ValueType};
use super::Names;
use json5_parser::JVal;
use crate::error::Result;
use super::json_array_to_rust::json_array_to_rust;


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
        JVal::Map(_map, span) => {
            Err(format!("{} Objects can't have objects {}", span.line_str(), names))?
        },
        JVal::Null(span) =>{
            Err(format!(r#"{} null must be ["type", null] {}"#, span.line_str(), names))?
        },
        JVal::Undefined(span) =>{
            Err(format!(r#"{} undefined must be ["type", undefined] {}"#, span.line_str(), names))?
        }
    }
}

pub fn json_item_to_rust_ref(name : &str, value_type : ValueType, v : &JVal, names : &Names) -> Result<RustValue> {
    let names = &names.append(name);
    match v {
        JVal::Bool(_, span) => {
            Err(format!("{} {} Ref object's members must be string or null {}", span.line_str(), span.slice(), names))?
        },
        JVal::Double(_, span)=>{
            Err(format!("{} {} Ref object's members must be string or null {}", span.line_str(), span.slice(), names))?
        }
        JVal::String(s, _) => {
            let s = s.to_string();
            Ok(RustValue::String(Qv::Val(s), value_type))
        },
        JVal::Array(_, span) => {
            Err(format!("{} {} Ref object's members must be string or null {}", span.line_str(), span.slice(), names))?
        },
        JVal::Map(_, span) => {
            Err(format!("{} {} Ref object's members must be string or null {}", span.line_str(), span.slice(), names))?
        },
        JVal::Null(_) =>{
            Ok(RustValue::String(Qv::Null, value_type))
        },
        JVal::Undefined(_)=>{
            Ok(RustValue::String(Qv::Undefined, value_type))
        }
    }
}

