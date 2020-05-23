use crate::indexmap::IndexMap;
use linked_hash_map::LinkedHashMap;
use crate::error::Result;
use std::collections::{BTreeMap};
use crate::structs::rust_value::RustValue;
use crate::structs::root_object::RustObject;
use crate::imp::json_to_rust::names::Names;
use crate::structs::rust_list::ListDef;

pub fn validate_list_defaults(list_name : &Names, list_def : &IndexMap<String, RustValue>, list_items : &LinkedHashMap<String, RustObject>, rename : &BTreeMap<String, String>) -> Result<()>{
    for (id, item) in list_items{
        //sabunはjsonからの変換時にはないので調べない・・・
        for (name, val) in &item.default {
            let name: &str = name;
            let sabun_val: &RustValue = val;
            //let name = rename.get(name).map(|n| n.as_str()).unwrap_or(name); json時点でrenameに頼るのは認めないことにする
            let def_val = list_def.get(name).ok_or_else(|| format!("{}'s default obj doesn't have {} .{}", list_name, name, id))?;

            if val_type_check(def_val, sabun_val, &list_name.append(name))? == false {
                Err(format!("list {}'s default value's type doesn't correspond to {}'s {}", list_name, get_id(item), name))?
            }
        }
    }
    return Ok(());
}

fn get_id(obj : &RustObject) -> String{
    obj.id.as_ref().map(|s| s.as_str()).unwrap_or_else(|| "no id").to_string()
}

fn val_type_check(l : &RustValue, r : &RustValue, names : &Names) -> Result<bool>{

    if l.type_num() != r.type_num(){
        return Ok(false);
    }
    if l.value_type().acceptable(&r.qv_type()) == false{
        return Ok(false);
    }


    if let RustValue::Array(_, l_at, _) = l {
        if let RustValue::Array(_, r_at, _) = r {
            if l_at.type_num() != r_at.type_num() {
                return Ok(false);
            }
        }
    }

    if let RustValue::List(l) = l {
        if let RustValue::List(r) = r {
            if l.list.len() != 0{
                Err(format!("{} inner list's default obj must not have any items", names))?
            }
            if let ListDef::InnerList = r.default{}
            else{
                Err(format!("{} inner list's default objects must be undefined", names))?
            }
            if let ListDef::Def(def) = &l.default {
                validate_list_defaults(names, &def.default, &r.list, &def.renamed);
            }
            else{
                Err(format!("{} default object's list's default object must be defined", names))?
            }
        }
    }
    return Ok(true);
}