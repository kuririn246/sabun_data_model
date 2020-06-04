use crate::structs::root_object::RootObject;
use crate::error::Result;
use crate::imp::json_to_rust::json_name::dot_chained_name;
use crate::imp::json_to_rust::names::Names;
use crate::structs::rust_value::RustValue;
use std::collections::{HashMap, HashSet};
use crate::structs::rust_list::ListItem;

///refしている対象が存在しているかを調べる
pub fn validate_ref_from_root(member_name : &str, dot_chained : &str, can_ref_old : bool, root : &RootObject, names : &Names) -> Result<()>{
    if dot_chained.is_empty(){
        Err(format!("{} {} is empty", names, member_name))?
    }
    if can_ref_old == false && root.old().contains(member_name){
        Err(format!("{} the root object's {} is old {}", names, member_name, dot_chained))?
    }

    match dot_chained_name(dot_chained){
        None =>{ Err(format!("{} {} is not a dot-chained name {}", names, member_name, dot_chained))? }
        Some(v) =>{
            if let Some(id) = v.get(0){
                if let Some(first_data) = root.default().get(member_name){
                    if let RustValue::Data(d) = first_data{
                        validate_ref_recursive(d.list(), id, &v[1..], can_ref_old,d.old(), names,
                                               &Names::new(member_name), dot_chained)?;
                        return Ok(())
                    } else{
                        Err(format!("{} the root object's {} was not Data {}", names, member_name, dot_chained))?
                    }
                } else{
                    Err(format!("{} the root object's {} was not found {}", names, member_name, dot_chained))?
                }
            } else{ unreachable!() }
        }
    }
}

fn validate_ref_recursive(map : &HashMap<String, ListItem>, id : &str, rest_dot_chained : &[&str], can_ref_old : bool,
                      old : &HashSet<String>, source_names : &Names, current_name : &Names, dot_chained : &str) -> Result<()> {
    if map.get(id).is_none() {
        Err(format!("{}'s {} was not found {} {}", current_name, id, source_names, dot_chained))?
    }
    if can_ref_old == false && old.contains(id) {
        Err(format!("{}'s {} is old {} {}", current_name, id, source_names, dot_chained))?
    }

    if rest_dot_chained.len() == 0 {
        return Ok(());
    }
    let name = rest_dot_chained[0];

    let value = if let Some(value) = map[id].values().get(name) { value } else {
        Err(format!("{}'s {} was not found {} {}", current_name, name, source_names, dot_chained))?
    };
    let current_name = &current_name.append(name);
    if rest_dot_chained.len() == 1 {
        Err(format!("{} doesn't have ID {} {}", current_name, source_names, dot_chained))?
    }
    let id = rest_dot_chained[1];
    if let RustValue::InnerData(d) = value {
        validate_ref_recursive(d.list(), id, &rest_dot_chained[2..], can_ref_old, d.old(), source_names, current_name, dot_chained)?;
        return Ok(());
    } else {
        Err(format!("{} is not Data {}", current_name, source_names))?
    }
}