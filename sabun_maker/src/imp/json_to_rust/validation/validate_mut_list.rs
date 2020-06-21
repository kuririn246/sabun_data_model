use crate::error::Result;
use crate::imp::json_to_rust::names::Names;
use crate::imp::json_to_rust::validation::validate_list_item::validate_list_item;
use crate::imp::structs::rust_list::MutListItem;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::HashM;

pub fn validate_mut_list(def : &ListDefObj, map : &HashM<u64, MutListItem>, root : &RootObject,
                         can_use_old: bool, names : &Names) -> Result<()>{
    for (idx, val) in map{
        let idx = idx.to_string();
        let names = &names.append(&idx);
        validate_list_item(def, val.values(), val.refs(), root, can_use_old, names)?;
    }
    return Ok(());
}