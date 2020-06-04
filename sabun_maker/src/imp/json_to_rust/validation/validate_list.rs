use crate::structs::root_object::{ListDefObj, RootObject};
use std::collections::HashMap;
use crate::structs::rust_list::ListItem;
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;
use crate::imp::json_to_rust::validation::validate_list_item::validate_list_item;
use crate::imp::json_to_rust::validation::validate_refs::validate_refs;

pub fn validate_list(def : &ListDefObj, data_vec : &Vec<ListItem>, root : &RootObject, names : &Names) -> Result<()>{
    for (idx, val) in data_vec.iter().enumerate(){
        let names = &names.append(&idx.to_string());
        validate_list_item(def.default(), val.values(), def.old(), root, names)?;
        validate_refs(def.refs(), val.refs(), root, names)?;
    }
    return Ok(());
}