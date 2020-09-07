use crate::{HashM, HashS};
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;
use crate::imp::json_to_rust::validation::validate_list_item::validate_list_item;
use crate::imp::json_to_rust::validation::validate_old_def_mem::validate_old_data_id;
use crate::imp::structs::rust_list::ConstItem;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::list_def_obj::ListDefObj;

pub fn validate_data(def : &ListDefObj, data_map : &HashM<String, ConstItem>, root : &RootObject, old : &HashS<String>,
                     can_use_old: bool, names : &Names) -> Result<()>{
    validate_old_data_id(old,data_map, names)?;

    for (name, val) in data_map{
        //name==old なものがあっても別にかまわない。消すと互換性が崩れるだろう
        validate_list_item(def, val.values(), val.refs(), root, can_use_old, &names.append(name))?
    }
    return Ok(());
}