use crate::rust_struct::{RustValue, Qv};
use super::Names;
use json5_parser::JVal;
//use serde_json::Value;
//use crate::json_array_to_rust::json_array_to_rust;
//use crate::json_to_rust::{Names, json_obj_to_rust2};

fn is_nullable(s : &str, names : &Names) -> Result<bool, String>{
    if let Some((b,_)) = super::json_name::is_possible_name(s){
        Ok(b)
    } else{
        Err(format!("{} is not a valid name {}", s, names.to_string()))
    }
}

pub fn json_item_to_rust(k : &str, v : &JVal, names : &Names) -> Result<RustValue, String> {
    todo!();
//    match v {
//        Value::Bool(b) => {
//            if is_nullable(k, names)? {
//                Ok(RustValue::NullableBool(Qv::Val(Some(*b))))
//            } else {
//                Ok(RustValue::Bool(Qv::Val(*b)))
//            }
//        },
//        Value::Number(num) => {
//
//            let num = num.as_f64().ok_or(format!("{} couldn't convert to f64 {}", num, names.to_string()))?; //numberがf64に変換できないなんてことないと思う・・・
//            if is_nullable(k, names)? {
//                Ok(RustValue::NullableNumber(Qv::Val(Some(num))))
//            } else {
//                Ok(RustValue::Number(Qv::Val(num)))
//            }
//        },
//        Value::String(s) => {
//            let s = s.to_string();
//            if is_nullable(k, names)? {
//                Ok(RustValue::NullableString(Qv::Val(Some(s))))
//            } else {
//                Ok(RustValue::String(Qv::Val(s)))
//            }
//        },
//        Value::Array(a) => {
//            let is_nullable = is_nullable(k, names)?;
//
//            let value = json_array_to_rust(a, is_nullable, names)?;
//            Ok(value)
//        },
//        Value::Object(o) => {
//            let obj = json_obj_to_rust2(o, names)?;
//
//            if is_nullable(k, names)? {
//                Ok(RustValue::NullableObject(Qv::Val(Some(obj))))
//            } else {
//                Ok(RustValue::Object(Qv::Val(obj)))
//            }
//        }
//        _ => { Err(format!("{} An item must be object or array or number or bool or string {}", v, names.to_string())) },
//    }
}


