use crate::rust_struct::RustValue;
use serde_json::Value;
use crate::json_array_to_rust::json_array_to_rust;
use crate::json_to_rust::{Names, json_obj_to_rust2};

fn is_nullable(s : &str, names : &Names) -> Result<bool, String>{
    if let Some((b,_)) = crate::json_name::is_possible_name(s){
        Ok(b)
    } else{
        Err(format!("{} is not a valid name {}", s, names.to_string()))
    }
}

pub fn json_item_to_rust(k : &str, v : &Value, names : &Names) -> Result<RustValue, String> {
    match v {
        Value::Bool(b) => {
            if is_nullable(k, names)? {
                Ok(RustValue::NullableBool(Some(*b)))
            } else {
                Ok(RustValue::Bool(*b))
            }
        },
        Value::Number(num) => {

            let num = num.as_f64().ok_or(format!("{} couldn't convert to f64 {}", num, names.to_string()))?; //numberがf64に変換できないなんてことないと思う・・・
            if is_nullable(k, names)? {
                Ok(RustValue::NullableNumber(Some(num)))
            } else {
                Ok(RustValue::Number(num))
            }
        },
        Value::String(s) => {
            let s = s.to_string();
            if is_nullable(k, names)? {
                Ok(RustValue::NullableString(Some(s)))
            } else {
                Ok(RustValue::String(s))
            }
        },
        Value::Array(a) => {
            let is_nullable = is_nullable(k, names)?;

            let value = json_array_to_rust(a, is_nullable, names)?;
            Ok(value)
        },
        Value::Object(o) => {
            let obj = json_obj_to_rust2(o, names)?;

            if is_nullable(k, names)? {
                Ok(RustValue::NullableObject(Some(obj)))
            } else {
                Ok(RustValue::Object(obj))
            }
        }
        _ => { Err(format!("{} An item must be object or array or number or bool or string {}", v, names.to_string())) },
    }
}


