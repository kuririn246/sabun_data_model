use std::collections::{HashMap, HashSet};
use crate::structs::rust_value::{RustValue, ExistenceType};
use crate::structs::root_object::{RootObject, ListDefObj};
use crate::imp::json_to_rust::names::Names;
use crate::error::Result;
use crate::imp::json_to_rust::validation::validate_data::validate_data;

pub fn validate_list_item(def : &ListDefObj, sabun_values : &HashMap<String, RustValue>, root : &RootObject, names : &Names) -> Result<()>{
    for (name, val) in sabun_values{
        if def.old().contains(name){
            Err(format!("{} {} is old", names, name))?
        }
        match def.default().get(name){
            Some(def) =>{
                if def.acceptable(val) == false{
                    Err(format!("{} {} the default value doesn't correspond to the list item's value", names, name))?
                }
                if let Some(def_obj) = def.inner_def(){
                    match def{
                        RustValue::InnerDataDef(def) => if let RustValue::InnerData(data) = val{
                            validate_data(def, data.list(), root, &names.append(name))
                        } else{ unreachable!(); }
                    }
                }

            },
            None =>{ Err(format!("{} there's no default members named {}", names, name))? }
        }
    }
    Ok(())
}