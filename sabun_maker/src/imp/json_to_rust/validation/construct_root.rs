use crate::rust_struct::{RustObject, RustValue, ValueType};
use crate::imp::json_to_rust::json_name::{json_name, NameType};
use std::collections::HashMap;
use crate::error::Result;

pub fn construct_root(root : RustObject, map : HashMap<String, RustValue>) -> Result<RustObject>{
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

    return Ok(root);
}