use crate::rust_struct::RustValue;
use serde_json::Value;
use crate::json_array_to_rust::json_array_to_rust;
use crate::json_to_rust::{json_obj_to_rust, Names};

fn is_possible_name<'a,'b>(s : &'a str, names : &'b Names) -> Result<(bool, &'a str), String>{
    if let Some(t) = crate::json_name::is_possible_name(s){
        Ok(t)
    } else{
        Err(format!("{} is not a valid name {}", s, names.to_string(s)))
    }
}

pub fn json_item_to_rust(k : &str, v : &Value, names : &Names) -> Result<RustValue, String> {
    match v {
        Value::Bool(b) => {
            let (is_nullable, name) = is_possible_name(k, names)?;
            if is_nullable {
                Ok(RustValue::NullableBool(Some(*b)))
            } else {
                Ok(RustValue::Bool(*b))
            }
        },
        Value::Number(num) => {
            let (is_nullable, name) = is_possible_name(k, names);
            let num = num.as_f64()?;
            if is_nullable {
                Ok(RustValue::NullableNumber(Some(num)))
            } else {
                Ok(RustValue::Number(num))
            }
        },
        Value::String(s) => {
            let (is_nullable, name) = is_possible_name(k, names);
            let s = s.to_string();
            if is_nullable {
                Ok(RustValue::NullableString(Some(s)))
            } else {
                Ok(RustValue::String(s))
            }
        },
        Value::Array(a) => {
            let (is_nullable, name) = is_possible_name(k);

            if let Some(value) = json_array_to_rust(a, is_nullable) {
                Ok(value)
            }
        },
        Value::Object(o) => {
            let (is_nullable, name) = is_possible_name(k);
            let obj = json_obj_to_rust(o)?;

            if is_nullable {
                Ok(RustValue::NullableObject(Some(obj)))
            } else {
                Ok(RustValue::Object(obj))
            }
        }
        _ => { panic!(); },
    }
}


