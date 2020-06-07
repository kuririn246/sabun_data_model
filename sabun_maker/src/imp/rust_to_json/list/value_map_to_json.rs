use std::collections::{BTreeMap};
use crate::structs::rust_value::RustValue;
use crate::structs::my_json::Value;
use crate::imp::rust_to_json::rust_value_to_json_value::rust_value_to_json_value;

pub(crate) fn value_map_to_json(map : &BTreeMap<String, RustValue>) -> BTreeMap<String, Value>{
    let mut result = BTreeMap::new();

    for (name,val) in map{
        let (name, val) = rust_value_to_json_value(val, name);
        result.insert(name, val);
    }

    return result;
}