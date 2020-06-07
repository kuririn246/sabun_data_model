use std::collections::{HashSet};
use crate::imp::json_to_rust::names::Names;
use crate::error::Result;
use crate::imp::json_to_rust::json_name::dot_chained_name;
use crate::imp::structs::def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;

pub(crate) fn validate_compatible(source_def : &ListDefObj, compatible : &HashSet<String>, root : &RootObject, can_use_old : bool, names : &Names) -> Result<()>{
    for dot_chained in compatible{
        match dot_chained_name(dot_chained){
            Some(v) =>{
                if v.len() == 0 {
                    Err(format!("{}'s compatible doesn't have valid name {}", names, dot_chained))?
                }
                let name = v[0];
                //compatibleはlist_defであってlist_defは旧バージョンから移行したデータには残っていないからcan_use_oldは関係ないはずだが一応
                if can_use_old == false && root.old().contains(name){
                    Err(format!("{} the root object's {} is old {}", names, name, dot_chained))?
                }
                if let Some(value) = root.default().get(name){
                    if let Some(def) = value.list_def() {
                        search_recursive(source_def, def, &v[1..], can_use_old, names, &Names::new(name), dot_chained)?
                    } else{
                        Err(format!("{} the root object's {} is not a list {}", names, name, dot_chained))?
                    }
                } else{
                    Err(format!("{} root doesn't have {} {}", names, name, dot_chained))?
                }
            }
            None =>{ Err(format!("{} {} is not a dot-chained name", names, dot_chained))? }
        }
    }
    return Ok(())
}

fn search_recursive(source_def : &ListDefObj, current_def : &ListDefObj, rest_dot_chained : &[&str], can_use_old : bool,
                    source_name : &Names, current_name : &Names, dot_chained : &str) -> Result<()> {
    if rest_dot_chained.len() == 0 {
        if source_def.compatible(current_def) {
            return Ok(())
        } else {
            Err(format!("{} is not compatible with {} {}", current_name, source_name, dot_chained))?
        }
    }

    let name = rest_dot_chained[0];
    let current_name = &current_name.append(name);

    if can_use_old == false && current_def.old().contains(name) {
        Err(format!("{} is old {} {}", current_name, source_name, dot_chained))?
    }

    let value = if let Some(item) = current_def.default().get(name) { item } else {
        Err(format!("{} was not found {} {}", current_name, source_name, dot_chained))?
    };

    let def= if let Some(v) = value.inner_def(){ v } else{
        Err(format!("{} is not a list {} {}", current_name, source_name, dot_chained))?
    };

    search_recursive(source_def, def, &rest_dot_chained[1..], can_use_old, source_name, current_name, dot_chained)?;
    return Ok(());
}
