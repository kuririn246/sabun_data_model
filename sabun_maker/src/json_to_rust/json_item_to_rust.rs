use crate::rust_struct::{RustValue, Qv, ValueType};
use super::Names;
use json5_parser::JVal;
use crate::error::Result;
use crate::json_to_rust::json_obj_to_rust::json_obj_to_rust;



pub fn json_item_to_rust(name : &str, value_type : ValueType, v : &JVal, names : &Names) -> Result<RustValue> {
    match v {
        JVal::Bool(b, _) => {
            Ok(RustValue::Bool(Qv::Val(*b), value_type))
        },
        JVal::Int(num, _) =>{
            Ok(RustValue::Number(Qv::Val(*num as f64), value_type))
        },
        JVal::Double(f, _)=>{
            Ok(RustValue::Number(Qv::Val(*f), value_type))
        }
        JVal::String(s, _) => {
            let s = s.to_string();
            Ok(RustValue::String(Qv::Val(s), value_type))
        },
        JVal::Array(a, _) => {
//            let is_nullable = is_nullable(k, names)?;

            //let value = json_array_to_rust(a, is_nullable, names)?;
            //Ok(value)
            todo!();
        },
        JVal::Map(map, _) => {
            let obj = json_obj_to_rust(map, &names.append(name))?;
            Ok(RustValue::Object(Qv::Val(obj), value_type))
        },
        JVal::Null(_) =>{
             Err(format!(r#"{} null must be ["type", null] {}"#, v.line_col(), names))?
        },
    }
}


