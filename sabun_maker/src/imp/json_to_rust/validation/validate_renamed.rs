use crate::structs::rust_object::RustObject;
use crate::error::Result;
use crate::structs::rust_value::RustValue;
use std::collections::HashMap;
use crate::imp::json_to_rust::names::Names;

pub fn validate_renamed(obj : &RustObject, names : &Names) -> Result<()>{
    if obj.default.is_none(){ return Ok(()); }

    let def = obj.default.as_ref().unwrap();
    for (key, value) in &obj.renamed{
        if def.contains_key(value) == false{
            Err(format!(r#"{} Renamed "{}->{}": {} doesn't exist"#, names, key, value, value))?
        }
    }
    return Ok(());
}