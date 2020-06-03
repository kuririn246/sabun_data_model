use std::collections::HashMap;
use crate::structs::rust_value::{RustValue, ExistenceType};
use crate::structs::root_object::RootObject;
use crate::imp::json_to_rust::names::Names;
use crate::error::Result;

pub fn validate_list_item(def_map : &HashMap<String, RustValue>, sabun_values : &HashMap<String, RustValue>, root : &RootObject, names : &Names) -> Result<()>{
    for (name, val) in sabun_values{
        match def_map.get(name){
            Some(def) =>{
                if def.acceptable(val) == false{
                    Err(format!("{} {} the default value doesn't correspond to the list item's value", names, name))?
                }
                if let Some(def_obj) = def.inner_def(){

                }

            },
            None =>{ Err(format!("{} there's no default members named {}", names, name))? }
        }
    }
    Ok(())
}