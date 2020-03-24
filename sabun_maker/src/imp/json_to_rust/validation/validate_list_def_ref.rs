use std::collections::BTreeMap;
use crate::error::Result;
use indexmap::IndexMap;
use crate::structs::rust_value::RustValue;
use crate::structs::ref_value::RefValue;

pub fn validate_list_def_ref(
    list_name : &str,
    list_def_ref : &IndexMap<String, RefValue>,
    root_def : &IndexMap<String, RustValue>,
    root_rename : &BTreeMap<String, String>) -> Result<()>{

    for (ref_name, _value) in list_def_ref{
        let renamed = root_rename.get(ref_name).map(|n| n.as_str()).unwrap_or(ref_name);

        if let Some(value) = root_def.get(renamed){
            match value{
                RustValue::List(_l) =>{},
                _ =>{ Err(format!("list {}'s default object's ref member {} doesn't correspond to the root object", list_name, renamed))? }
            }
        } else{
            Err(format!("list {}'s default object's ref member {} doesn't correspond to the root object", list_name, renamed))?
        }
    }
    return Ok(())
}