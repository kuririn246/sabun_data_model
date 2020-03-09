use super::json_name::{is_valid_name};
use super::names::{Names};
use std::collections::HashMap;
use json5_parser::JVal;

pub fn get_rename(v : &JVal, names : &Names) -> Result<HashMap<String,String>, String> {
    todo!();
//    let v = v.as_array().ok_or(format!("Rename must be an array. {}", names.to_string()))?;
//    let mut m : HashMap<String, String> = HashMap::new();
//    for item in v{
//        let s = item.as_str().ok_or(format!("{} must be string {}", item, names.to_string()))?;
//        let ss : Vec<&str> = s.split("->").collect();
//        if ss.len() != 2{ return Err(format!(r#"Rename must be "...->..." "#)); }
//
//        if is_valid_name(ss[0]) == false{
//            return Err(format!("{} is not a valid name  {}", ss[0], names.to_string()));
//        }
//        if is_valid_name(ss[1]) == false{
//            return Err(format!("{} is not a valid name  {}", ss[0], names.to_string()));
//        }
//
//        m.insert(ss[0].to_string(), ss[1].to_string());
//    }
//    Ok(m)
}