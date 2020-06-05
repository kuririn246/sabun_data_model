use crate::structs::root_object::{ListDefObj, RootObject};
use std::collections::{HashSet};
use crate::structs::rust_list::{MutListItem};
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;
use crate::imp::json_to_rust::validation::validate_list_item::validate_list_item;
use crate::imp::json_to_rust::validation::validate_compatible::validate_compatible;
use linked_hash_map::LinkedHashMap;

pub fn validate_mut_list(def : &ListDefObj, map : &LinkedHashMap<u64, MutListItem>, compatible : &HashSet<String>, root : &RootObject,
                         can_use_old: bool, names : &Names) -> Result<()>{
    validate_compatible(def, compatible, root, can_use_old, names)?;

    for (idx, val) in map{
        let idx = idx.to_string();
        let names = &names.append(&idx);
        validate_list_item(def, val.values(), val.refs(), root, can_use_old, names)?;
    }
    return Ok(());
}