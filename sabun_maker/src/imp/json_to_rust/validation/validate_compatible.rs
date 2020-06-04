use crate::structs::root_object::{ListDefObj, RootObject};
use std::collections::{HashSet};
use crate::imp::json_to_rust::names::Names;
use crate::error::Result;
use crate::imp::json_to_rust::json_name::dot_chained_name;
use crate::structs::rust_value::RustValue;

pub fn validate_compatible(def : &ListDefObj, compatible : &HashSet<String>, root : &RootObject, names : &Names) -> Result<()>{
    for dot_chained in compatible{
        match dot_chained_name(dot_chained){
            Some(v) =>{
                if v.len() == 0 {
                    Err(format!("{}'s compatible doesn't have valid name {}", names, dot_chained))?
                }
                let name = v[0];
                if root.old().contains(name){
                    Err(format!("{} the root object's {} is old", names, name))?
                }
                if let Some(value) = root.default().get(name){
                    search_recursive(def, value, &v[1..], names, &Names::new(name))?
                } else{
                    Err(format!("{} root doesn't have {} {}", names, name, dot_chained))?
                }
            }
            None =>{ Err(format!("{} {} is not a dot-chained name", names, dot_chained))? }
        }
    }
    return Ok(())
}

fn search_recursive(def : &ListDefObj, value : &RustValue, rest_dot_chained : &[&str],
                    source_name : &Names, current_name : &Names) -> Result<()> {
    if rest_dot_chained.len() == 0 {
        let other = if let Some(other) = value.list_def() { other } else {
            Err(format!("{} is not a list {}", current_name, source_name))?
        };
        if def.compatible(other) {
            return Ok(())
        } else {
            Err(format!("{} is not compatible with {}", current_name, source_name))?
        }
    }

    let data = if let RustValue::InnerData(data) = value { data } else {
        Err(format!("{} is not Data {}", current_name, source_name))?
    };

    let name = rest_dot_chained[0];
    let current_name = &current_name.append(name);

    if data.old().contains(name) {
        Err(format!("{} is old {}", current_name, source_name))?
    }

    let item = if let Some(item) = data.list().get(name) { item } else {
        Err(format!("{} was not found {}", current_name, source_name))?
    };
    if rest_dot_chained.len() == 1 {
        Err(format!("{} doesn't have ID {}", current_name, source_name))?
    }
    let name = rest_dot_chained[1];
    let current_name = &current_name.append(name);

    let value = if let Some(value) = item.values().get(name) { value } else {
        Err(format!("{} was not found {}", current_name, source_name))?
    };
    search_recursive(def, value, &rest_dot_chained[2..], source_name, current_name)?;
    return Ok(());
}
