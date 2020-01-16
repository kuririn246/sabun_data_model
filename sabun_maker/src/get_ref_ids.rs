use serde_json::Value;
use crate::json_name::{is_valid_name};
use std::collections::BTreeMap;
use crate::json_to_rust::{Names};

pub fn get_ref_ids(v : &Value, names : &Names) -> Result<BTreeMap<String,String>, String> {
    let v = v.as_object().ok_or(format!("RefIDs must be an object. {}", names.to_string()))?;
    let mut m : BTreeMap<String, String> = BTreeMap::new();
    for (k,v) in v{
        if is_valid_name(k) == false{
            return Err(format!("{} is not a valid name for RefIDs {}", k, names.to_string()));
        }
        let v = v.as_str().ok_or(format!("{} is not string {}", v, names.to_string()))?;
        m.insert(k.to_string(), v.to_string());
    }
    Ok(m)
}

//fn get_name(name : &str) -> Option<String>{
//    let name_type = json_name(name)?;
//    match &name_type{
//        NameType::Normal => return Some(name_type),
//        _ => return None,
//    };
//}