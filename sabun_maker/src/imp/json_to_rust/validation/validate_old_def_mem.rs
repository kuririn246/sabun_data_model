use std::collections::{HashSet, HashMap};
use crate::imp::json_to_rust::names::Names;
use crate::structs::ref_value::RefValue;
use crate::structs::rust_list::ListItem;
use crate::error::Result;

pub fn validate_old_def_mem<T>(old : &HashSet<String>, map : &HashMap<String, T>, names : &Names) -> Result<()>{
    for name in old{
        if map.contains_key(name) == false{
            Err(format!("{} Old {} does not exist in the Default", names, name))?
        }
    }
    Ok(())
}

pub fn validate_old_ref_def(old : &HashSet<String>, ref_def : &HashMap<String, RefValue>, names : &Names) -> Result<()>{
    for name in old{
        if ref_def.contains_key(name) == false{
            Err(format!("{} Old {} does not exist in the Ref", names, name))?
        }
    }
    Ok(())
}

pub fn validate_old_data_id(old : &HashSet<String>, data_map : &HashMap<String, ListItem>, names : &Names) -> Result<()> {
    for name in old{
        if data_map.contains_key(name) == false{
            Err(format!("{} Old's ID {} does not exist in the Data", names, name))?
        }
    }
    Ok(())
}