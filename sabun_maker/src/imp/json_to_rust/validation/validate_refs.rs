use crate::structs::root_object::{RootObject, RefDefObj};
use std::collections::{HashMap};
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;
use crate::structs::ref_value::RefValue;
use crate::imp::rust_to_json::name_with_suffix::name_with_suffix;
use crate::structs::qv::{Qv};
use crate::structs::rust_value::RustValue;

pub fn validate_refs(def : &RefDefObj, sabun : &HashMap<String, RefValue>, root : &RootObject, can_use_old: bool, names : &Names) -> Result<()>{
    if def.is_enum(){
       if sabun.len() != 1{
           Err(format!("{} one of the Enum's member must be defined", names))?
       }
    }

    for (name, sab_val) in sabun{
        if can_use_old == false && def.old().contains(name) {
            Err(format!("{} {} is old", names, name))?
        }
        match def.refs().get(name){
            Some(h) =>{
                if h.acceptable(sab_val) == false{
                    Err(format!("{} {} {} is not valid for {}", names, name, sab_val.value_js_string(), name_with_suffix(name, h.value_type())))?
                }
                match sab_val.value() {
                    Qv::Val(id) =>{
                        if id.is_empty(){
                            Err(format!("{} ref {} is empty", names, name))?
                        }
                        match root.default().get(name) {
                            Some(RustValue::Data(d)) => {
                                if d.list().get(id).is_none() {
                                    Err(format!("{}'s {} was not found {}", name, id, names))?
                                }
                                if can_use_old == false && d.old().contains(id) {
                                    Err(format!("{}'s {} is old {}", name, id, names))?
                                }
                                continue;
                            },
                            Some(_) => {
                                Err(format!("{} the root object's {} was not Data", names, name))?
                            },
                            None => {
                                Err(format!("{} the root object's {} was not found", names, name))?
                            }
                        }
                    },
                    _ =>{
                        //nullとかundefinedとかは有効な値である
                    }
                }
            },
            None =>{ Err(format!("{} there's no default ref members named {}", names, name))? }
        }
    }
    return Ok(());
}