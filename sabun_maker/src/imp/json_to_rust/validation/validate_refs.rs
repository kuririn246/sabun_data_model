use crate::structs::root_object::{ListDefObj, RootObject, RefDefObj};
use std::collections::{HashMap, HashSet};
use crate::structs::rust_list::ListItem;
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;
use crate::imp::json_to_rust::validation::validate_list_item::validate_list_item;
use crate::structs::ref_value::RefValue;

pub fn validate_refs(def : &RefDefObj, sabun : &HashMap<String, RefValue>, root : &RootObject, names : &Names) -> Result<()>{
    for (name, val) in sabun{
        if def.old().contains(name) {
            Err(format!("{} {} is old", names, name))?
        }

        def.refs()
        //validate_list_item(def.default(), val.values(), root, &names.append(name))?
    }
    return Ok(());
}