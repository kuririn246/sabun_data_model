use super::Names;
use json5_parser::JVal;
use crate::error::Result;
use super::json_array_to_rust::json_array_to_rust;
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::rust_value::{RustValue};
use crate::imp::structs::qv::Qv;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::rust_string::RustString;


pub fn json_item_to_rust(name : &str, value_type : VarType, v : &JVal, names : &Names) -> Result<RustValue> {
    let names = &names.append(name);
    match v {
        JVal::Bool(b, _) => {
            Ok(RustValue::Param(RustParam::Bool(Qv::Val(*b)), value_type))
        },
        JVal::Double(f, _)=>{
            Ok(RustValue::Param(RustParam::Float(Qv::Val(*f)), value_type))
        },
        JVal::Int(i, _) =>{
            Ok(RustValue::Param(RustParam::Int(Qv::Val(*i)), value_type))
        },
        JVal::String(s, _) => {
            let s = s.to_string();
            Ok(RustValue::Param(RustParam::String(Qv::Val(RustString::new(s))), value_type))
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

pub fn json_item_to_rust_ref(name : &str, value_type : VarType, v : &JVal, names : &Names) -> Result<RustValue> {
    let names = &names.append(name);
    match v {
        JVal::Bool(_, span) => {
            Err(format!("{} {} Ref object's members must be string or null {}", span.line_str(), span.slice(), names))?
        },
        JVal::Double(_, span)=>{
            Err(format!("{} {} Ref object's members must be string or null {}", span.line_str(), span.slice(), names))?
        },
        JVal::Int(_, span)=>{
            Err(format!("{} {} Ref object's members must be string or null {}", span.line_str(), span.slice(), names))?
        },
        JVal::String(s, _) => {
            let s = s.to_string();
            Ok(RustValue::Param(RustParam::String(Qv::Val(RustString::new(s))), value_type))
        },
        JVal::Array(_, span) => {
            Err(format!("{} {} Ref object's members must be string or null {}", span.line_str(), span.slice(), names))?
        },
        JVal::Map(_, span) => {
            Err(format!("{} {} Ref object's members must be string or null {}", span.line_str(), span.slice(), names))?
        },
        JVal::Null(_) =>{
            Ok(RustValue::Param(RustParam::String(Qv::Null), value_type))
        },
        JVal::Undefined(_)=>{
            Ok(RustValue::Param(RustParam::String(Qv::Undefined), value_type))
        }
    }
}

