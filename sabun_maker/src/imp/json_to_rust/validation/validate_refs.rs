use crate::structs::root_object::{ListDefObj, RootObject, RefDefObj};
use std::collections::{HashMap, HashSet};
use crate::structs::rust_list::ListItem;
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;
use crate::imp::json_to_rust::validation::validate_list_item::validate_list_item;
use crate::structs::ref_value::RefValue;
use crate::imp::rust_to_json::name_with_suffix::name_with_suffix;

pub fn validate_refs(def : &RefDefObj, sabun : &HashMap<String, RefValue>, root : &RootObject, names : &Names) -> Result<()>{
    for (name, val) in sabun{
        if def.old().contains(name) {
            Err(format!("{} {} is old", names, name))?
        }

        match def.refs().get(name){
            Some(h) =>{
                if h.acceptable(val) == false{
                    Err(format!("{} {} {} is not valid for {}", names, name, val.value_string(), name_with_suffix(name, h.value_type())))?
                }
            },
            None =>{ Err(format!("{} there's no default ref members named {}", names, name))? }
        }
    }
    return Ok(());
}