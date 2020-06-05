use crate::structs::root_object::{RootObject, RefDefObj};
use std::collections::{HashMap};
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;
use crate::structs::ref_value::RefValue;
use crate::imp::rust_to_json::name_with_suffix::name_with_suffix;
use crate::structs::qv::{Qv};
use crate::imp::json_to_rust::validation::validate_ref_from_root::validate_ref_from_root;

pub fn validate_refs(def : &RefDefObj, sabun : &HashMap<String, RefValue>, root : &RootObject, can_use_old: bool, names : &Names) -> Result<()>{
    if def.is_enum(){
       if sabun.len() != 1{
           Err(format!("{} one of the Enum's member must be defined", names))?
       }
    }

    for (name, val) in sabun{
        if can_use_old == false && def.old().contains(name) {
            Err(format!("{} {} is old", names, name))?
        }

        match def.refs().get(name){
            Some(h) =>{
                if h.acceptable(val) == false{
                    Err(format!("{} {} {} is not valid for {}", names, name, val.value_js_string(), name_with_suffix(name, h.value_type())))?
                }
                match h.value() {
                    Qv::Val(dot_chained) =>{
                        if dot_chained.is_empty(){
                            continue;
                        }
                        validate_ref_from_root(name, dot_chained, can_use_old, root, &names.append(name))?
                    },
                    _ =>{}
                }
            },
            None =>{ Err(format!("{} there's no default ref members named {}", names, name))? }
        }
    }
    return Ok(());
}