use indexmap::IndexMap;
use linked_hash_map::LinkedHashMap;
use crate::error::Result;
use std::collections::{BTreeMap};
use crate::structs::rust_value::RustValue;
use crate::structs::rust_object::RustObject;

pub fn validate_list_sabuns(list_name : &str, list_def : &IndexMap<String, RustValue>, list_items : &LinkedHashMap<String, RustObject>, rename : &BTreeMap<String, String>) -> Result<()>{
    for (id, item) in list_items{
        for (name, val) in &item.sabun {
            let name: &str = name;
            let sabun_val: &RustValue = val;

            let name = rename.get(name).map(|n| n.as_str()).unwrap_or(name);
            let def_val = list_def.get(name).ok_or_else(|| format!("{}'s default obj doesn't have {} .{}", list_name, name, id))?;

            if val_type_check(def_val, sabun_val) == false {
                Err(format!("list {}'s default value's type doesn't correspond to {}'s {}", list_name, get_id(item), name))?
            }
        }
    }
    return Ok(());
}

fn get_id(obj : &RustObject) -> String{
    obj.id.as_ref().map(|s| s.as_str()).unwrap_or_else(|| "no id").to_string()
}

fn val_type_check(l : &RustValue, r : &RustValue) ->bool{

    if l.type_num() != r.type_num(){
        return false
    }
    if l.value_type().acceptable(&r.qv_type()) == false{
        return false;
    }


    if let RustValue::Array(_, l_at, _) = l {
        if let RustValue::Array(_, r_at, _) = r {

            if l_at.type_num() != r_at.type_num() {
                return false
            }
        }
    }
    return true;
}