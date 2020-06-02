use crate::imp::json_to_rust::json_name::{json_name, NameType};
use std::collections::HashMap;
use crate::error::Result;
use crate::structs::root_object::{ RootObject};
use crate::structs::rust_value::RustValue;
use crate::structs::value_type::ValueType;

pub fn construct_root(root : RootObject, map : HashMap<String, RustValue>, validation : bool) -> Result<RootObject>{
    let mut root = root;
    for (key, value) in map{
        let name = json_name(&key).ok_or_else(|| format!("filename:{} is not a valid name", &key))?;
        match name {
            NameType::Name(name, ValueType::Normal) => {
                root.default.insert(name, value);
            },
            _=>{ Err(format!("{} is not a valid name", &key))?; }
        }
    }
    if validation{
        //validate_root(&root)?
    }

    return Ok(root);
}