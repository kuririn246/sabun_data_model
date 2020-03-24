use crate::my_json::Value;
use crate::error::Result;
use crate::rust_to_json_new_default;
use crate::structs::rust_object::RustObject;

pub fn default_to_json(obj : &RustObject) -> Result<Value>{
    let mut result : Vec<Value> = vec![];
    result.push(Value::String("Default".to_string()));
    let nd = rust_to_json_new_default(obj)?;
    result.push(nd);
    return Ok(Value::Array(result));
}