use crate::error::Result;
use crate::rust_to_json_new_default;
use crate::structs::root_object::RustObject;
use crate::structs::my_json::Value;

pub fn default_to_json(obj : &RustObject, root : &RustObject) -> Result<Value>{
    let mut result : Vec<Value> = vec![];
    result.push(Value::String("Default".to_string()));
    let nd = rust_to_json_new_default(obj, None, root)?;
    result.push(nd);
    return Ok(Value::Array(result));
}