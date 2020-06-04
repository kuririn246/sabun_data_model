use crate::structs::root_object::RootObject;
use crate::error::Result;
use crate::imp::json_to_rust::json_name::dot_chained_name;
use crate::imp::json_to_rust::names::Names;
use crate::structs::rust_value::RustValue;
use std::collections::{HashMap, HashSet};
use crate::structs::rust_list::ListItem;

pub fn validate_ref_from_root(member_name : &str, dot_chained : &str, root : &RootObject, names : &Names) -> Result<()>{
    if dot_chained.is_empty(){
        Err(format!("{} is empty", names))?
    }
    match dot_chained_name(dot_chained){
        None =>{ Err(format!("{} {} is not a dot-chained name", names, dot_chained))? }
        Some(v) =>{
            if let Some(id) = v.get(0){
                if root.old().contains(id){
                    Err(format!("{} the root object's {} is old {}", names, s, dot_chained))?
                }

                if let Some(first_data) = root.default().get(member_name){
                    if let Some(RustValue::Data(d)) = first_data{
                        validate_recursive(d.list(), &v, d.old(), names, &Names::new("."))?
                    } else{
                        Err(format!("{} the root object's {} was not Data {}", names, member_name, dot_chained))?
                    }
                } else{
                    Err(format!("{} the root object's {} was not found {}", names, member_name, dot_chained))?
                }

            } else{ Err(format!("{} unknown error validate_ref_from_root {}", names, dot_chained))? }
        }
    }
    return Ok(())
}

fn validate_recursive(map : &HashMap<String, ListItem>, dot_chained : &[&str], old : &HashSet<String>, source_names : &Names, current_names : &Names) -> Result<()>{
    if dot_chained.len() == 0{ unreachable!(); }

    let name = dot_chained[0];

    if dot_chained.len() == 1{
        if map.get(name).is_none(){
            Err(format!("{}'s {} was not found {} {}", current_names, name, source_names, dot_chained))?
        }
        if old.contains(name){
            Err(format!("{}'s {} is old {} {}", current_names, name, source_names, dot_chained))?
        }
        return Ok(());
    }

    match map.get(name){
        RustValue::InnerData(d) =>{
            validate_recursive(d.list(), &dot_chained[1..], d.old(), source_names, &current_names.append(name))?;
            return Ok(());
        }
        _ =>{  Err(format!("{}'s {} is not InnerData {} {}", current_names, name, source_names, dot_chained))? }
    }


}