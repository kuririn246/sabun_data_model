use crate::structs::root_object::{ListDefObj, RootObject};
use std::collections::{HashMap};
use crate::structs::rust_list::ListItem;
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;
use crate::imp::json_to_rust::validation::validate_list_item::validate_list_item;

pub fn validate_data(def : &ListDefObj, data_map : &HashMap<String, ListItem>, root : &RootObject, can_use_old: bool, names : &Names) -> Result<()>{
    for (name, val) in data_map{
        //name==old なものがあっても別にかまわない。消すと互換性が崩れるだろう
        validate_list_item(def, val.values(), val.refs(), root, can_use_old, &names.append(name))?
    }
    return Ok(());
}