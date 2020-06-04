use crate::structs::root_object::RootObject;
use crate::error::Result;
use crate::imp::json_to_rust::json_name::dot_chained_name;
use crate::imp::json_to_rust::names::Names;
use crate::structs::rust_value::RustValue;
use std::collections::{HashMap, HashSet};
use crate::structs::rust_list::ListItem;

///refしている対象が存在しているかを調べる
pub fn validate_ref_from_root(member_name : &str, dot_chained : &str, root : &RootObject, names : &Names) -> Result<()>{
    if dot_chained.is_empty(){
        Err(format!("{} {} is empty", names, member_name))?
    }
    if root.old().contains(member_name){
        Err(format!("{} the root object's {} is old", names, member_name))?
    }

    match dot_chained_name(dot_chained){
        None =>{ Err(format!("{} {} {} is not a dot-chained name", names, member_name, dot_chained))? }
        Some(v) =>{
            if let Some(id) = v.get(0){
                if let Some(first_data) = root.default().get(member_name){
                    if let RustValue::Data(d) = first_data{
                        validate_recursive(d.list(), id, &v[1..], d.old(), names, &Names::new(member_name))?
                    } else{
                        Err(format!("{} the root object's {} was not Data {}", names, member_name, dot_chained))?
                    }
                } else{
                    Err(format!("{} the root object's {} was not found {}", names, member_name, dot_chained))?
                }
            } else{ unreachable!() }
        }
    }
    return Ok(())
}

fn validate_recursive(map : &HashMap<String, ListItem>, id : &str, rest_dot_chained : &[&str], old : &HashSet<String>, source_names : &Names, current_name : &Names) -> Result<()> {
    if map.get(id).is_none() {
        Err(format!("{}'s {} was not found {}", current_name, id, source_names))?
    }
    if old.contains(id) {
        Err(format!("{}'s {} is old {}", current_name, id, source_names))?
    }

    if rest_dot_chained.len() == 0 {
        return Ok(());
    }
    let name = rest_dot_chained[0];

    if let Some(value) = &map[id].values().get(name){
        let current_name = &current_name.append(name);
        if rest_dot_chained.len() == 1{
            Err(format!("{} doesn't have ID {}", current_name, source_names))?
        }
        let id = rest_dot_chained[1];
        if let RustValue::InnerData(d) = value{
            validate_recursive(d.list(), id, &rest_dot_chained[2..], d.old(), source_names, current_name)?;
            return Ok(());
        } else{
            Err(format!("{} is not Data {}", current_name, source_names))?
        }
    } else{
        Err(format!("{}'s {} was not found {}", current_name, name, source_names))?
    }
}