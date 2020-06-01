use std::collections::BTreeMap;
use crate::structs::my_json::Value;

pub fn get_renamed(map : &BTreeMap<String, String>) -> Vec<Value>{
    let mut result : Vec<Value> = vec![];

    for (k,v) in map{
        result.push(Value::String(format!("{}->{}", k, v)))
    }

    return result;
}