use crate::my_json::Value;

pub fn get_include(vec : &Vec<String>) -> Vec<Value>{
    vec.iter().map(|s| Value::String(s.to_string())).collect()
}