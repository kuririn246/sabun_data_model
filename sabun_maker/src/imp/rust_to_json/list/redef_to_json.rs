use std::collections::BTreeMap;
use crate::structs::my_json::Value;

pub fn redef_to_json(map : &BTreeMap<String, String>) -> Value{
    let mut result : Vec<Value> = vec![ Value::String("Redef".to_string()) ];
    for (k,v) in map{
        result.push(
            Value::String(
                format!("{}->{}", k.to_string(), v.to_string())
            )
        )
    }
    return Value::Array(result);
}