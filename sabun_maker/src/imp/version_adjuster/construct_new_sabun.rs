use std::collections::BTreeMap;
use crate::structs::rust_value::RustValue;
use indexmap::IndexMap;

pub fn construct_new_sabun(rename : &BTreeMap<String, String>, sabun : IndexMap<String, RustValue>) -> IndexMap<String, RustValue>{
    let mut result : IndexMap<String, RustValue> = IndexMap::new();
    for (key, value) in sabun{
        let name = rename.get(&key).unwrap_or(&key);
        result.insert(name.to_string(), value);
    }
    return result;
}