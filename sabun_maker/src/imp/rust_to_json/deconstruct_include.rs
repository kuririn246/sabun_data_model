use crate::structs::json_file::JsonDir;
use std::collections::{BTreeMap};
use crate::structs::my_json::Value;

pub fn deconstruct_include(root : Value) -> JsonDir{
    let mut result : BTreeMap<String, Value> = BTreeMap::new();
    let inc = get_include(&root);
    if let Value::Map(map) = root{
        let mut map = map;
        for name in inc {
            if let Some(value) = map.remove(&name){
                result.insert(name, value);
            }
        }
        map.remove("Include");
        result.insert("root".to_string(), Value::Map(map));

    }
    return JsonDir(result);
}

fn get_include(root : &Value) -> Vec<String>{
    let mut result : Vec<String> = vec![];

    if let Value::Map(map) = root {
        if let Some(item) = map.get("Include") {
            if let Value::Array(vec) = item {
                for v in vec {
                    if let Value::String(s) = v {
                        result.push(s.to_string());
                    }
                }
            }
        }
    }
    return result;
}