use crate::imp::json_to_rust::json_name::{json_name, NameType};
use std::collections::HashMap;
use crate::error::Result;
use crate::imp::json_to_rust::validation::validate_lists::validate_lists;
use crate::structs::rust_object::RustObject;
use crate::structs::rust_value::RustValue;
use crate::structs::value_type::ValueType;
use crate::imp::json_to_rust::validation::validate_renamed::validate_renamed;
use crate::imp::json_to_rust::names::Names;
use crate::imp::json_to_rust::validation::validate_root::validate_root;

pub fn construct_root(root : RustObject, map : HashMap<String, RustValue>, validation : bool) -> Result<RustObject>{
    let mut root = root;
    for (key, value) in map{
        let name = json_name(&key).ok_or_else(|| format!("{} is not a valid name", &key))?;
        match name {
            NameType::Name(name, ValueType::Normal) => {
                root.include.push(key);
                root.insert_default(name, value);
            },
            _=>{ Err(format!("{} is not a valid name", &key))?; }
        }
    }
    if validation{
        validate_root(&root)?
    }

    return Ok(root);
}