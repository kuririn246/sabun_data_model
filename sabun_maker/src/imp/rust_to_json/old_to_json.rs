use std::collections::{ BTreeSet};
use crate::structs::my_json::Value;

///long : ["Old", "hoge", "huga"] short : ["hoge", "huga"]
pub fn old_to_json_short(s : &BTreeSet<String>) -> Value{
    Value::Array(s.iter().map(|s| Value::String(s.to_string())).collect())
}

///long : ["Old", "hoge", "huga"] short : ["hoge", "huga"]
pub fn old_to_json_long(s : &BTreeSet<String>) -> Value {
    string_set_to_json("Old", s)
}

///[s, "hoge", "huga"]
pub fn string_set_to_json(tag : &str, s : &BTreeSet<String>) -> Value{
    let mut vec = vec![Value::String(tag.to_string())];
    vec.extend(s.iter().map(|s| Value::String(s.to_string())));
    Value::Array(vec)
}

