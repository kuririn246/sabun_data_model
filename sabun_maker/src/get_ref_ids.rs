use serde_json::Value;
use crate::json_name::SystemNames;
use std::collections::BTreeMap;
use crate::json_to_rust::Names;

pub fn get_ref_ids(v : &Value, names : &Names) -> Result<BTreeMap<String,String>, String>{
    let v = v.as_object().ok_or(format!("RefIDs must be an object. {}", names.to_string()))?;

}