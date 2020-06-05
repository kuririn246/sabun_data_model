use crate::structs::root_object::{ListDefObj, RootObject};
use std::collections::{HashSet};
use crate::imp::json_to_rust::names::Names;
use crate::error::Result;
use crate::imp::json_to_rust::json_name::dot_chained_name;
use crate::structs::rust_value::RustValue;

pub fn validate_compatible(def : &ListDefObj, compatible : &HashSet<String>, root : &RootObject, can_use_old : bool, names : &Names) -> Result<()>{
    for dot_chained in compatible{
        match dot_chained_name(dot_chained){
            Some(v) =>{
                if v.len() == 0 {
                    Err(format!("{}'s compatible doesn't have valid name {}", names, dot_chained))?
                }
                let name = v[0];
                //compatibleはlist_defであってlist_defは旧バージョンから移行したデータには残っていないはずだが一応・・・
                if can_use_old == false && root.old().contains(name){
                    Err(format!("{} the root object's {} is old {}", names, name, dot_chained))?
                }
                if let Some(value) = root.default().get(name){
                    search_recursive(def, value, &v[1..], can_use_old,names, &Names::new(name), dot_chained)?
                } else{
                    Err(format!("{} root doesn't have {} {}", names, name, dot_chained))?
                }
            }
            None =>{ Err(format!("{} {} is not a dot-chained name", names, dot_chained))? }
        }
    }
    return Ok(())
}

fn search_recursive(def : &ListDefObj, value : &RustValue, rest_dot_chained : &[&str], can_use_old : bool,
                    source_name : &Names, current_name : &Names, dot_chained : &str) -> Result<()> {
    if rest_dot_chained.len() == 0 {
        let other = if let Some(other) = value.list_def() { other } else {
            Err(format!("{} is not a list {} {}", current_name, source_name, dot_chained))?
        };
        if def.compatible(other) {
            return Ok(())
        } else {
            Err(format!("{} is not compatible with {} {}", current_name, source_name, dot_chained))?
        }
    }

    let data = if let RustValue::InnerData(data) = value { data } else {
        Err(format!("{} is not Data {} {}", current_name, source_name, dot_chained))?
    };

    let name = rest_dot_chained[0];
    let current_name = &current_name.append(name);

    if can_use_old == false && data.old().contains(name) {
        Err(format!("{} is old {} {}", current_name, source_name, dot_chained))?
    }

    let item = if let Some(item) = data.list().get(name) { item } else {
        Err(format!("{} was not found {} {}", current_name, source_name, dot_chained))?
    };
    if rest_dot_chained.len() == 1 {
        Err(format!("{} doesn't have ID {} {}", current_name, source_name, dot_chained))?
    }
    let name = rest_dot_chained[1];
    let current_name = &current_name.append(name);

    let value = if let Some(value) = item.values().get(name) { value } else {
        Err(format!("{} was not found {} {}", current_name, source_name, dot_chained))?
    };
    search_recursive(def, value, &rest_dot_chained[2..], can_use_old, source_name, current_name, dot_chained)?;
    return Ok(());
}
