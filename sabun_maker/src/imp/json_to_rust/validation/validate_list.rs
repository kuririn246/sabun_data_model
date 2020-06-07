use crate::structs::root_object::{ListDefObj, RootObject};
use crate::structs::rust_list::ListItem;
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;
use crate::imp::json_to_rust::validation::validate_list_item::validate_list_item;

pub(crate) fn validate_list(def : &ListDefObj, data_vec : &Vec<ListItem>, root : &RootObject, can_use_old: bool, names : &Names) -> Result<()>{
    for (idx, val) in data_vec.iter().enumerate(){
        let idx = idx.to_string();
        let names = &names.append(&idx);
        validate_list_item(def, val.values(), val.refs(), root, can_use_old, names)?;
    }
    return Ok(());
}