use serde_json::Value;
use crate::rust_struct::RustValue;

pub fn rust_value_to_json_value(v : &RustValue) -> (Value, String){
    match v{
        RustValue::Bool()
    }
}